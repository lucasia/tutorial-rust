use log::debug;

mod boxes;

fn main() {
    env_logger::init();

    debug!("====== boxes ======");
    boxes::boxes();
}