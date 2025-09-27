mod ownership;
mod references;
mod slices;
use log::info;

fn main() {
    env_logger::init();

    info!("========== 4.3 slice type ==========");
    slices::slices();

    info!("========== 4.2 references ==========");
    references::references();

    info!("========== 4.1 ownership ==========");
    ownership::ownership();
}
