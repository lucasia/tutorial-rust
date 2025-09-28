use log::debug;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn recoverable() {
    debug!("recoverable $$$");

    create_dotenv();
    let user_result = read_username();
    match user_result {
        Ok(user) => debug!("Found user: {user}"),
        Err(e) => debug!("Error: {e:?}")
    }
    panic_at_the_disco();
}

fn read_username() -> Result<String, io::Error> {
    let username_file_result = File::open("assets/user.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn panic_at_the_disco() -> File {
    File::open("disco!")
        .expect("Panic at the taco bell!")
}

fn create_dotenv() {
    let file_path = "assets/.env";

    let greeting_file_result = File::open(file_path);

    // TODO: replace this with closures after chapter 13.
    let greeting_file: File = match greeting_file_result {
        Ok(file) => {
            debug!("Found the file.  Huzzah!!");
            file
        }

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {e:?}"),
            }
            _ => {
                panic!("Error opening file: {error:?}")
            }
        },
    };

    assert!(greeting_file.metadata().unwrap().is_file());
}
