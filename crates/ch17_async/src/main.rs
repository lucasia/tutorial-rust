mod async_concur;
mod futures;
mod yield_example;
mod timeout;

use log::debug;

fn main() {
    env_logger::init();

    debug!("========== futures ==========");
    futures::futures("https://www.rust-lang.org", "https://doc.rust-lang.org/");

    debug!("========== async concurrency ==========");
    async_concur::async_concur();

    debug!("========== yield ==========");
    yield_example::yield_example();

    debug!("========== timeout ==========");
    timeout::timeout_example();

    debug!("========== streams ==========");
    // TODO - streams https://doc.rust-lang.org/book/ch17-04-streams.html
}
