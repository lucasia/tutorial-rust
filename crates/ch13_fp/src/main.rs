use log::debug;

mod closures;
mod iterators;

fn main() {
    env_logger::init();

    debug!("====== closures ======");
    closures::closures();

    debug!("====== iterators ======");
    iterators::iterators();
}
