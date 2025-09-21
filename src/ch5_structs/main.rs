mod rectangles;
mod struct_def;

use log::info;

fn main() {
    env_logger::init();

    info!("========== 5.3 syntax ==========");

    info!("========== 5.2 rectangle ==========");
    rectangles::rectangles();

    info!("========== 5.1 define structs ==========");
    struct_def::struct_def();
}
