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

    debug!("parsed: {parsed_class:#?}");
}
