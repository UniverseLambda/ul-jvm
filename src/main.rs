use std::path::Path;

use anyhow::{Context, bail};
use binrw::BinRead;
use class::{JvmUnit, parser::ClassFile};
use exec::JvmExecEnv;
use log::{debug, info};

mod class;
mod exec;
mod types;

fn main() {
    env_logger::init();

    info!("uLambda's JVM version {}", env!("CARGO_PKG_VERSION"));
    let mut jvm_exec_env = JvmExecEnv::new();

    let first_unit = load_unit("Main", &[".".to_string()]).expect("loading Main.class");

    let mut done = jvm_exec_env.add_unit(first_unit);

    while !done {
        for class in jvm_exec_env.missing_units() {
            let jvm_unit = load_unit(&class, &[".".to_string()])
                .expect(&format!("unable to load class file {class}"));

            done = jvm_exec_env.add_unit(jvm_unit);
        }
    }
}

pub fn load_unit(full_name: &str, class_path: &[String]) -> anyhow::Result<JvmUnit> {
    debug!("Looking up class file for {full_name} in {class_path:?}...");
    let mut full_path = None;

    for current_dir in class_path {
        let current_path = Path::new(current_dir)
            .join(full_name)
            .with_extension("class");

        if current_path.is_file() {
            full_path = Some(current_path);
            break;
        }
    }

    let Some(full_path) = full_path else {
        bail!("no file found in class path for {full_name}");
    };

    debug!("Found class file for {full_name} at {full_path:?}");

    let mut content = std::fs::OpenOptions::new()
        .read(true)
        .open(full_path)
        .context("opening class file")?;

    let parsed_class = ClassFile::read(&mut content).expect("UNABLE TO PARSE FILE");

    info!("Dumping parsed class file...");
    std::fs::write(
        format!("{}-class_dump.json", full_name.replace('/', ".")),
        serde_json::to_string_pretty(&parsed_class).unwrap(),
    )
    .context("write parsed class file dump JSON")?;

    debug!("Putting everything nice and cosy");
    let jvm_unit =
        JvmUnit::from_class_file(parsed_class).context("creating JVM unit from class file")?;

    info!("Dumping processed JVM unit...");
    std::fs::write(
        format!("{}-jvm_unit_dump.json", full_name.replace('/', ".")),
        serde_json::to_string_pretty(&jvm_unit).unwrap(),
    )
    .context("write unit dump JSON")?;

    Ok(jvm_unit)
}
