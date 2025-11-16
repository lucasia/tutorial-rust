use log::debug;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn recoverable() {
    debug!("recoverable $$$");
    create_dotenv();

    // read user
    let user_result = read_username();
    match user_result {
        Ok(user) => debug!("Found user: {user}"),
        Err(e) => debug!("Error: {e:?}"),
    }

    // read a char
    let char =
        last_char_of_first_line("the rain\n in spain.").expect("Should have found the char!");
    assert_eq!('n', char);

    // keep last, panics!
    panic_at_the_disco();
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// verbose to show ?.  can be simplified to fs::read_to_string("path")
fn read_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("assets/user.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn panic_at_the_disco() -> File {
    File::open("disco!").expect("Panic at the taco bell!")
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
            },
            _ => {
                panic!("Error opening file: {error:?}")
            }
        },
    };

    assert!(greeting_file.metadata().unwrap().is_file());
}
