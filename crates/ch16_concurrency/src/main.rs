use log::debug;

mod messages;
mod threads;
mod shared_state;

fn main() {
    env_logger::init();

    debug!("========== threads ==========");
    threads::threads();

    debug!("========== messages ==========");
    messages::messages();

    debug!("========== messages ==========");
    shared_state::shared_state();

}
