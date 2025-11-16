mod generics;
mod lifetimes;
mod traits;

fn main() {
    env_logger::init();

    generics::generics();
    traits::traits();
    lifetimes::lifetimes();
}
