mod enum_def;

use log::info;

fn main() {
    env_logger::init();

    info!("========== 5.1 define enums ==========");
    enum_def::enum_def();
}