/*
  重构以改进模块化、错误处理
*/

// 命令行读取文件
#![allow(unused)]
use std::env;
use std::fs;

//简单将解析处理逻辑抽离
pub fn test() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
}
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

//使用结构体存储配置，更清晰参数意义
pub fn test1() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config1(&args);
    println!("Searching for {} \t In file {}", config.query, config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
pub struct Config {
    pub query: String,
    pub file_path: String,
}
fn parse_config1(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let file_path: String = args[2].clone();

    Config { query, file_path }
}

// 使用 impl 来关联函数

pub fn test2() {
    use std::process;
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {} \t In file {}", config.query, config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
impl Config {
    pub fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Config { query, file_path }
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Config { query, file_path })
    }
}
