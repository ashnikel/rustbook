use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive (Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    fn config_new_ok() {
        let args = [String::from("minigrep"),
                    String::from("query"),
                    String::from("poem.txt"),
        ];
        let conf = Config { query: String::from("query"),
                            filename: String::from("poem.txt")
        };
        let result = Config::new(&args).unwrap();
        assert_eq!(conf, result);
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn config_new_not_enough_args() {
        let args = [String::from("minigrep")];
        Config::new(&args).unwrap();
    }

    #[test]
    fn run_ok() {
        let conf = Config { query: String::from("query"),
                            filename: String::from("poem.txt")
        };

        assert_eq!((), run(conf).unwrap());
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn run_file_not_found() {
        let conf = Config { query: String::from("query"),
                            filename: String::from("no-such-file.txt")
        };

        run(conf).unwrap();
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}