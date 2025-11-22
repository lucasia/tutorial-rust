use log::{debug};

mod closures;

fn main() {
    env_logger::init();

    debug!("====== closures ======");
    closures::closures();

    // TODO - 13.2 Iterators next up !

}

