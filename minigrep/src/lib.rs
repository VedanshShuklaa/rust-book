use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub files: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let files = args[2..].to_vec();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            files,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for i in config.files.iter() {
        let mut results: Vec<&str> = Vec::new();
        let content = fs::read_to_string(i)?;
        if config.ignore_case {
            results.append(&mut search_case_insensitive(&config.query, &content));
        } else {
            results.append(&mut search(&config.query, &content));
        }

        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
