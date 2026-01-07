#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use learn_macro_01::create_function;
use rand::Rng;

create_function!(foo);

#[learn_macro_02::hello]
fn test2() {}
#[learn_macro_02::my_attr(a, b = 1)]
fn test22() {}

#[learn_macro_02::trace]
fn test31() {
    print!("in test31 \t");
}
#[learn_macro_02::log()]
fn test32() {
    println!("in test32");
}
#[learn_macro_02::log2(level = 2)]
fn test33() {
    println!("in test33");
}

use learn_macro_03::MyDebug;
#[derive(learn_macro_03::MyDebug)]
struct Point {
    x: i32,
    value: i32,
}

pub fn run() {
    let res = test1().unwrap_or_else(|e| e);
    println!("运行测试后：{}", res);
    test31();
    print_name(); // 调用宏生成的函数
    // println!("-------------------");
    // test32();
    // println!("-------------------");
    // test33();
    // println!("-------------------");

    let p = Point { x: 10, value: 20 };
    println!("Point debug: {}", p.my_debug());
}

fn test1() -> Result<String, String> {
    let res = foo().map_err(|e| format!("失败: {}", e))?.ok_or("为空".to_string())?;
    print!("test1 res is {res} \t ");
    Ok("成功".to_string())
}
