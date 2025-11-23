use ch12_cli::{search, search_case_insensitive};
use log::{debug, error, info};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    env_logger::init();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        error!("Usage: cargo run -- <query> <file_path>.  Error: {}", err);
        process::exit(1);
    });

    info!(
        "Searching for '{}' in path '{}'",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    debug!("file contents is {}", contents);

    if config.ignore_case {
        search_case_insensitive(&config.query, &contents).for_each(|line| println!("{}", line));
    } else {
        search(&config.query, &contents).for_each(|line| println!("{}", line));
    };

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Requires query and file_path as arguments!");
        // }
        args.next(); // binary name = not needed
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string missing"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path missing"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
