/*
  改进之前的I/O项目
*/
#![allow(dead_code)]
use std::env;

pub fn run() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    test1(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });
}

struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        // let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

//1.使用迭代器消除clone
fn test1(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = std::fs::read_to_string(config.file_path)?;
    let results: Vec<&str> = search(&config.query, &contents);
    for line in results {
        println!("{line}");
    }
    Ok(())
}
