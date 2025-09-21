mod enum_def;
mod match_flow;

use log::info;

fn main() {
    env_logger::init();

    info!("========== 5.2 match flow ==========");
    match_flow::match_flow();

    info!("========== 5.1 define enums ==========");
    enum_def::enum_def();
}
