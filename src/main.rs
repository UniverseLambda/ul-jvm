use log::info;

mod class;
mod types;

fn main() {
    env_logger::init();

    info!("uLambda's JVM version {}", env!("CARGO_PKG_VERSION"));
}
