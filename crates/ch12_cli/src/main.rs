use std::{env, fs, process};
use std::error::Error;
use log::{debug, error, info};
use ch12_cli::{search, search_case_insensitive};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        error!("Usage: cargo run -- <query> <file_path>.  Error: {}", err);
        process::exit(1);
    });


    info!("Searching for '{}' in path '{}'", config.query, config.file_path);

    if let Err(e) = run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    debug!("file contents is {}", contents);

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Requires query and file_path as arguments!");
        }

        debug!("{:?}", args);
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}