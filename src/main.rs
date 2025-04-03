use binrw::BinRead;
use class::{JvmUnit, parser::ClassFile};
use log::{debug, info, trace};

mod class;
mod types;

fn main() {
    env_logger::init();

    info!("uLambda's JVM version {}", env!("CARGO_PKG_VERSION"));

    let mut content = std::fs::OpenOptions::new()
        .read(true)
        .open("Main.class")
        .unwrap();

    let parsed_class = ClassFile::read(&mut content).expect("UNABLE TO PARSE FILE");

    info!("Dumping parsed class file...");
    std::fs::write(
        "class_dump.json",
        serde_json::to_string_pretty(&parsed_class).unwrap(),
    )
    .unwrap();

    debug!("Putting everything nice and cosy");
    let jvm_unit = JvmUnit::from_class_file(parsed_class).unwrap();

    info!("Dumping processed JVM unit...");
    std::fs::write(
        "jvm_unit_dump.json",
        serde_json::to_string_pretty(&jvm_unit).unwrap(),
    )
    .unwrap();
}
