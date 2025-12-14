mod futures;

use std::{env, process};
use log::{debug, error, info};
use trpl::Either;
use crate::futures::page_title;

fn main() {
    env_logger::init();

    debug!("========== futures ==========");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        error!("Usage: cargo run -- <url1> <url2>");
        process::exit(1);
    }
    futures::futures(&args[1], &args[2])

}