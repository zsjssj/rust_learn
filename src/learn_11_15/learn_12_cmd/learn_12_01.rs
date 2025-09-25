// 接收命令行参数
#![allow(unused)]
use std::env;
use std::fs;

pub fn test() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
