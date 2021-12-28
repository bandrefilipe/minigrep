use std::error::Error;
use std::fs;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args)?;
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        Ok(config)
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creation_requires_at_least_three_arguments() {
        let mut args = vec!["one".to_string(), "two".to_string()];
        assert_eq!(Config::new(&args), Err("Not enough arguments"));

        args.push("three".to_string());
        assert_eq!(
            Config::new(&args),
            Ok(Config {
                query: "two".to_string(),
                filename: "three".to_string(),
            }),
        );
    }

    const TEST_CONTENTS: &str = "\
    I'm nobody! Who are you?\n\
    Are you nobody, too?\n\
    Then there's a pair of us - don't tell!\n\
    They'd banish us, you know.\n\
    \n\
    How dreary to be somebody!\n\
    How public, like a frog\n\
    To tell your name the livelong day\n\
    To an admiring bog!\
    ";

    #[test]
    fn search_finds_one_line() {
        let query = "frog";
        assert_eq!(
            search(query, TEST_CONTENTS),
            vec!["How public, like a frog"]
        );
    }

    #[test]
    fn search_finds_multiple_lines() {
        let query = "body";
        assert_eq!(
            search(query, TEST_CONTENTS),
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!",
            ]
        );
    }

    #[test]
    fn search_finds_nothing() {
        let query = "monomorphization";
        let empty_vec: Vec<&str> = vec![];
        assert_eq!(search(query, TEST_CONTENTS), empty_vec);
    }
}
