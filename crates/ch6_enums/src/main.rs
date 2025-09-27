mod enum_def;
mod if_let;
mod match_flow;

use log::info;

fn main() {
    env_logger::init();

    info!("========== 6.3 if let ==========");
    if_let::if_let();

    info!("========== 6.2 match flow ==========");
    match_flow::match_flow();

    info!("========== 6.1 define enums ==========");
    enum_def::enum_def();
}
