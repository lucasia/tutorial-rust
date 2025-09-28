use log::debug;
use std::fs::File;

pub fn recoverable() {
    debug!("recoverable $$$");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => {
            debug!("Found the file.  Huzzah!!");
            file
        }
        Err(error) => panic!("Error opening file: {error:?}"),
    };

    assert!(greeting_file.metadata().unwrap().is_file());
}
