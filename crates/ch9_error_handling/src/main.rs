mod recoverable;
mod unrecoverable;

fn main() {
    env_logger::init();

    unrecoverable::unrecoverable();
    recoverable::recoverable();
}
