//! 模式语法
//参考：https://kaisery.github.io/trpl-zh-cn/ch19-02-refutability.html
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    test1();
}
fn test1() {
    let a: Option<u8> = None;
    if let Some(x) = a {
        println!("a is {}", x);
    } else {
        println!("a is None");
    };
}
