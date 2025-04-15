use std::{io::stdout, path::Path};

use anyhow::{Context, bail};
use binrw::BinRead;
use class::{JvmUnit, parser::ClassFile};
use class_container::read_container;
use either::Either;
use exec::{JvmExecEnv, thread::JvmThread};
use log::{debug, error, info, warn};
use types::JvmTypeDescriptor;

mod class;
mod class_container;
mod exec;
mod types;

fn main() {
    env_logger::init();

    info!("uLambda's JVM version {}", env!("CARGO_PKG_VERSION"));
    let mut jvm_exec_env = JvmExecEnv::new();

    let first_unit = load_unit("Main", &[".".to_string()], true).expect("loading Main.class");

    let mut done = jvm_exec_env.add_unit(first_unit, true);

    while !done {
        for class in jvm_exec_env.missing_units() {
            let jvm_unit = load_unit(
                &class,
                &[
                    ".".to_string(),
                    "/usr/lib/jvm/jre/jmods/java.base.jmod".to_string(),
                    "/usr/lib/jvm/default-java/jmods/java.base.jmod".to_string(),
                ],
                false,
            )
            .unwrap_or_else(|_| panic!("unable to load class file {class}"));

            done = jvm_exec_env.add_unit(jvm_unit, false);
        }
    }

    let start_class = jvm_exec_env
        .start_class
        .as_ref()
        .expect("no start class found");

    let (_, main_method) = start_class
        .methods
        .iter()
        .find(|(name, method)| {
            *name == "main"
                && method.ret_type().is_none()
                && method.is_static()
                && method.parameters()
                    == [JvmTypeDescriptor::Array(Box::new(
                        JvmTypeDescriptor::Class("java/lang/String".into()),
                    ))]
        })
        .expect("no main method in the specified class");

    let mut main_thread = JvmThread::new(start_class.clone(), main_method);

    if let Err(err) = main_thread.run(&jvm_exec_env) {
        error!("error on main thread: {err}");

        main_thread.dump_to(stdout()).unwrap();

        Err(err).expect("error on main thread")
    };
}

pub fn load_unit(full_name: &str, class_path: &[String], dump: bool) -> anyhow::Result<JvmUnit> {
    // TODO: cache units location

    debug!("Looking up class file for {full_name} in {class_path:?}...");
    let mut source = None;

    for current_dir in class_path {
        if current_dir.ends_with(".jar")
            || current_dir.ends_with(".JAR")
            || current_dir.ends_with(".jmod")
            || current_dir.ends_with(".JMOD")
        {
            let jar_file = match read_container(current_dir) {
                Ok(v) => v,
                Err(e) => {
                    warn!("unable to read JAR file {current_dir}: {e}. Skipping...");
                    continue;
                }
            };

            if jar_file.has_unit(full_name) {
                source = Some(Either::Right(jar_file.read_class_file(full_name)?));
                break;
            }
        }

        let current_dir = Path::new(current_dir);

        // TODO: Handle when this is a dir
        // current_dir.is_dir();

        let current_path = current_dir.join(full_name).with_extension("class");

        if current_path.is_file() {
            source = Some(Either::Left(
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(current_path)
                    .context("opening class file")?,
            ));
            break;
        }
    }

    let Some(source) = source else {
        bail!("no JVM unit in class path for {full_name}");
    };

    debug!("Found class file for {full_name}");

    let parsed_class = match source {
        Either::Left(mut v) => ClassFile::read(&mut v)?,
        Either::Right(mut v) => ClassFile::read(&mut v)?,
    };

    if dump {
        info!("Dumping parsed class file...");
        std::fs::write(
            format!("{}-class_dump.json", full_name.replace('/', ".")),
            serde_json::to_string_pretty(&parsed_class).unwrap(),
        )
        .context("write parsed class file dump JSON")?;
    }

    debug!("Putting everything nice and cosy");
    let jvm_unit =
        JvmUnit::from_class_file(parsed_class).context("creating JVM unit from class file")?;

    if dump {
        info!("Dumping processed JVM unit...");
        std::fs::write(
            format!("{}-jvm_unit_dump.json", full_name.replace('/', ".")),
            serde_json::to_string_pretty(&jvm_unit).unwrap(),
        )
        .context("write unit dump JSON")?;
    }

    Ok(jvm_unit)
}
