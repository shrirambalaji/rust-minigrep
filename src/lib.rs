use std::{error::Error, fs};

const OPTION_CASE_INSENSITIVE_LONG: &str = "--case-insensitive";
const OPTION_CASE_INSENSITIVE_SHORT: &str = "-i";

#[derive(Debug)]
pub struct ConfigOptions {
    pub case_insensitive: bool,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filepath: &'a str,
    pub options: Option<ConfigOptions>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // iter.partition partitions an iterator into two buckets (A, B)
        // A -> items that match predicate
        // B -> items that dont match predicate
        let (minigrep_options, minigrep_args): (Vec<_>, Vec<_>) =
            args.into_iter().partition(|arg| arg.starts_with("-"));

        let query = minigrep_args[1];
        let filepath = minigrep_args[2];
        let options = None;
        let mut config = Self {
            filepath: &filepath,
            query: &query,
            options,
        };

        // why is the && necessary?
        if minigrep_options.contains(&&OPTION_CASE_INSENSITIVE_LONG.to_owned())
            || minigrep_options.contains(&&OPTION_CASE_INSENSITIVE_SHORT.to_owned())
        {
            config.options = Some(ConfigOptions {
                case_insensitive: true,
            })
        }

        Ok(config)
    }
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.to_owned());
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(config.filepath).expect("Something went wrong while reading the file");

    println!("With text: \n {:?}", file_contents);

    let results = search(config.query, &file_contents);
    for hit in results {
        println!("{}", hit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_search_query_in_contents() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn should_return_no_results_for_search() {
        let query = "hello";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let results: Vec<String> = Vec::new();
        assert_eq!(results, search(query, contents));
    }
}
