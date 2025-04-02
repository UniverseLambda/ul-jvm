use binrw::BinRead;
use class::parser::ClassFile;
use log::{debug, info};

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

    debug!("Dumping parsed class file...");
    std::fs::write(
        "class_dump.json",
        serde_json::to_string_pretty(&parsed_class).unwrap(),
    )
    .unwrap();
}
