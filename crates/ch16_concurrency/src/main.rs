use log::debug;

mod messages;
mod threads;

fn main() {
    env_logger::init();

    debug!("========== threads ==========");
    // threads::threads();

    debug!("========== messages ==========");
    messages::messages();
}
