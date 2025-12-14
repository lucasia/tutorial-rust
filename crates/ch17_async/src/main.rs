mod async_concur;
mod futures;
use log::debug;

fn main() {
    env_logger::init();

    debug!("========== futures ==========");
    futures::futures("https://www.rust-lang.org", "https://doc.rust-lang.org/");

    debug!("========== async concurrency ==========");
    async_concur::async_concur();
}
