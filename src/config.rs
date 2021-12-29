#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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
}
