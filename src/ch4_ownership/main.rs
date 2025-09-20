mod ownership;
mod references;
mod slices;
use log::info;

fn main() {
    env_logger::init();

    // TODO - 4.2 ref and borrowing and 4.3 the slice type
    info!("========== 4.3 slice type ==========");

    slices::slices();

    info!("========== 4.2 references ==========");

    references::references();

    info!("========== 4.1 ownership ==========");

    ownership::ownership();
}

