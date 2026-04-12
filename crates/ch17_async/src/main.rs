mod async_concur;
mod futures;
mod streams;
mod timeout;

use log::debug;

fn main() {
    env_logger::init();

    debug!("========== futures ==========");
    futures::futures("https://www.rust-lang.org", "https://doc.rust-lang.org/");

    debug!("========== timeout ==========");
    timeout::timeout_example();

    // TODO - 17.5 - A closer look at the traits for async.  mostly text, skipping for now.
}
