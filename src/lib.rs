mod config;
mod search;

use crate::config::Config;
use crate::search::search;
use std::error::Error;
use std::fs;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args)?;
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
