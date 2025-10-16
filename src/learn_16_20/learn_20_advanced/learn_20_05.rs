//!宏
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-05-macros.html
#![allow(dead_code)]
#![allow(unused_variables)]
pub fn run() {
    // test1();
    // test2();
    test4();
}

//宏和函数的区别
fn test1() {}

//使用 macro_rules! 的声明宏用于通用元编程
fn test2() {
    let v: Vec<u32> = vec![1, 2, 3];
}

//vec! 宏的一个稍微简化的定义
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

//用于从属性生成代码的过程宏
//fn test3() {}
// use proc_macro;
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
//     //处理并输出
//     input
// }

//
//如何编写自定义 derive 宏
fn test4() {
    Pancakes::hello_macro();
}
//1.手动实现 trait
// use hello_macro::HelloMacro;
// struct Pancakes;
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("\x1b[31mHello, Macro! My name is Pancakes!\x1b[0m");
//     }
// }

//2.使用自定义 derive 宏
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
#[derive(HelloMacro)]
struct Pancakes;
