mod panic;
mod result;

pub struct Driver {
    name: String,
    age: u8,
}

impl Driver {
    pub fn new(name: String, age: u8) -> Driver {
        if (age < 18) {
            panic!("Driver must be 18 or older");
        }

        Driver {name, age}
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    panic::pain_main();

    result::result_main();

    Ok(())
}

