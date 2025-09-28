use std::error::Error;

mod recoverable;
mod unrecoverable;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    unrecoverable::unrecoverable();
    recoverable::recoverable();

    Ok(())
}
