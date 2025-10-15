//!高级函数与闭包
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-04-advanced-functions-and-closures.html

#![allow(dead_code)]
#![allow(unused_variables)]
pub fn run() {
    test1();
    // test2();
}
//函数指针
fn test1() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
