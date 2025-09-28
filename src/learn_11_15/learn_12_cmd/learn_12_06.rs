/*
  将错误信息输出到标准错误而不是标准输出
  eprintln! 宏与 println! 宏类似，但它将输出写入标准错误（stderr）而不是标准输出（stdout）。这在处理错误消息时很有用，因为它允许将错误消息与正常的程序输出分开。
  例如：
  eprintln!("Error: Could not open file");
  这将把错误消息 "Error: Could not open file" 输出到标准错误流。
  这样做的好处是，用户或调用程序可以更容易地捕获和处理错误消息，而不会干扰正常的程序输出。
  这种做法在命令行工具和脚本中尤为常见，因为它们通常需要区分正常输出和错误输出，以便进行适当的处理和日志记录。
  通过这种方式，程序可以更好地与其他工具和脚本集成，提高了错误处理的灵活性和可靠性。
  另外，使用 eprintln! 宏还可以确保错误消息在终端或日志文件中以醒目的方式显示，帮助用户更快地识别和解决问题。
  总之，eprintln! 宏是 Rust 提供的一个方便的工具，用于将错误消息输出到标准错误流，从而实现更好的错误处理和用户体验。
  参考：https://doc.rust-lang.org/std/macro.eprintln.html
  学习参考：https://kaisery.github.io/trpl-zh-cn/ch12-06-writing-to-stderr-instead-of-stdout.html
*/

//忽略未使用
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// use super::learn_12_03::Config;
struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case: bool = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    test1(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });
}

fn test1(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = std::fs::read_to_string(config.file_path)?;
    let results: Vec<&str> = if config.ignore_case { search_case_insensitive(&config.query, &contents) } else { search(&config.query, &contents) };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
