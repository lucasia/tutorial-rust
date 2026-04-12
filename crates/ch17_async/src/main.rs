mod async_concur;
mod futures;
mod timeout;
mod streams;

use log::debug;

fn main() {
    env_logger::init();

    debug!("========== futures ==========");
    futures::futures("https://www.rust-lang.org", "https://doc.rust-lang.org/");

    debug!("========== timeout ==========");
    timeout::timeout_example();
}
