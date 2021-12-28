use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    process::exit(match minigrep::run(&args) {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("error: {}", error);
            1
        }
    });
}
