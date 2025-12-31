//!所有可能会用到模式的位置
//参考：https://kaisery.github.io/trpl-zh-cn/ch19-01-all-the-places-for-patterns.html
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    // test1();
    test2();
}
//【match 分支】 、【if let 条件表达式】
fn test1() {
    let x: Option<i32> = Option::<i32>::Some(50);
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse::<u8>();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

//【while let 条件循环】、【for 循环】、【函数参数】
fn test2() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3, 4, 5] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}
