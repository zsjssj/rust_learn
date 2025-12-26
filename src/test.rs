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
    let res = test1().unwrap_or("aaaaaaaaa".to_string());
    println!("run res is {}", res);
    test2();
    test22();
    test31();
    test32();
    test33();

    let p = Point { x: 10, value: 20 };
    println!("Point debug: {}", p.my_debug());
}

fn test1() -> Result<String, String> {
    let res = foo()
        .map_err(|e| format!("获取云端重建地址失败: {}", e))?
        .ok_or(" 为空")?;
    println!("test1 res is {res}");
    Ok("success".to_string())
}
