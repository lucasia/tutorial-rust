use std::fs::File;
use std::io::{self, Read, ErrorKind};
use chrono::Local;
use rand::Rng;

pub fn result_main() {
    random_txt_file_name();

    error_handler_example1();
    error_handler_example2();

    error_handler_example3();

    propagate_error_example1();
    propagate_error_example2();

    let mut string_lines = String::new();
    string_lines.push_str("Hello World\n");
    string_lines.push_str("Goodbye Moon\n");

    let result = last_char_of_first_line(string_lines.as_str());
    println!("Last char of first line: {:?}", result);
}

fn error_handler_example1() {
    let greeting_file_result = File::open(random_txt_file_name());

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        //Err(e) => panic!("Uh oh!! {e}"),

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(random_txt_file_name()) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file!! {error}"),
            },
            _ => panic!("Uh oh!! {error}"),
        }
    };

    println!("{:?}", greeting_file);
}

fn error_handler_example2() {
    File::open(random_txt_file_name()).unwrap_or_else(|error| {
        match File::create(random_txt_file_name()) {
            Ok(file) => {
                println!("Gotcha!");
                file
            },
            Err(error) => panic!("No way, Jose! {error}")
        }
    });
}

fn error_handler_example3() {
    //let greeting_file = File::open(random_txt_file_name()).unwrap(); // will panic  if error

    let file_name = random_txt_file_name();
    // let greeting_file = File::open(&file_name).expect(format!("Failed to open file '{}'", &file_name).as_str()); // will panic with message
}

fn random_txt_file_name() -> String {
    format!("test-{}.txt", random_name())
}

fn random_name() -> String {
    let timestamp = Local::now().timestamp();
    let random_number: u32 = rand::thread_rng().gen();

    let random_str = format!("{timestamp}_{random_number}");

    dbg!(&random_str);

    random_str
}

fn propagate_error_example1() -> Result<String, io::Error> {

    let user_file_result = File::open("user.txt");

    let mut username_file = match user_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn propagate_error_example2() -> Result<String, io::Error> {

    let mut username_file = File::open(random_txt_file_name())?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}