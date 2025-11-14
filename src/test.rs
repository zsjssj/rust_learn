#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::path::Path;
pub fn run() {
    let p1 = std::env::current_dir().unwrap();
    println!("Current exe path: {}", p1.display());
    let p2 = p1.join("learn_21_01/assets/hello.html");
    println!("Path: {}", p2.display());
    let _content = std::fs::read_to_string(p2).unwrap();
    println!("File content: {}", _content);
}
