use log::debug;

mod messages;
mod shared_state;
mod threads;

fn main() {
    env_logger::init();

    debug!("========== threads ==========");
    threads::threads();

    debug!("========== messages ==========");
    messages::messages();

    debug!("========== messages ==========");
    shared_state::shared_state();
}
