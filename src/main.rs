use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for '{}' in file '{}'", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("Contents:\n{}", contents);
}
