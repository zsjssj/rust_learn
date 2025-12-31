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
    println!("in test31");
}
#[learn_macro_02::log()]
fn test32() {
    println!("in test32");
}
#[learn_macro_02::log2(level = 2)]
fn test33() {
    println!("in test33");
}

trait MyDebug {
    fn my_debug(&self) -> String;
}

#[derive(learn_macro_02::MyDebug)]
struct Point {
    x: i32,
    value: i32,
}

pub fn run() {
    let res = test1().unwrap_or_else(|e| e);
    println!("运行测试后：{}", res);
    // test2();
    // println!("-------------------");
    // test22();
    // println!("-------------------");
    // test31();
    // println!("-------------------");
    // test32();
    // println!("-------------------");
    // test33();
    // println!("-------------------");

    let p = Point { x: 10, value: 20 };
    println!("Point debug: {}", p.my_debug());

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn test1() -> Result<String, String> {
    let res = foo().map_err(|e| format!("失败: {}", e))?.ok_or("为空".to_string())?;
    println!("test1 res is {res}");
    Ok("success".to_string())
}
