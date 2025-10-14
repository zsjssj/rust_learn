//!所有可能会用到模式的位置
//参考：https://kaisery.github.io/trpl-zh-cn/ch19-03-pattern-syntax.html
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test41();
    // test42();
    // test43();
    // test5();
    // test51();
    // test52();
    // test6();
    test7();
}
//匹配字面值
fn test1() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
//匹配命名变量
fn test2() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");
}

//【多个模式】、【通过 ..= 匹配值范围】
fn test3() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

//解构并分解值【解构结构体】
fn test4() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
struct Point {
    x: i32,
    y: i32,
}

//【解构枚举】
fn test41() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//【解构嵌套的结构体和枚举】
fn test42() {
    let msg = MessageC::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        MessageC::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        MessageC::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum MessageC {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

//解构结构体和元组
fn test43() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}, inches: {inches}, x: {x}, y: {y}");
}

//使用 _ 忽略整个值
fn test5() {
    foo(3, 4);
}
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
//使用嵌套的 _ 忽略部分值
fn test51() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    //忽略特定值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
    //通过在变量名开头加 _ 来忽略未使用的变量
    let _x = 5;
    let y = 10; //没有下划线，编译器会警告未使用，因为加了全局属性 #![allow(unused_variables)]所以不会报警告

    //_x 仍会将值绑定到变量，而 _ 则完全不会绑定值
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{s:?}"); //报错 borrow of moved value: `s`
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");
}

//用 .. 忽略剩余值
fn test52() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin: Point = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    //元组中使用 ..
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    //下方使用报错
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {second}")
    //     }
    // }
}

//匹配守卫提供的额外条件
fn test6() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    //测试外部变量的值
    let x: Option<i32> = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    //优先级：匹配守卫 if y 作用于 4、5、6，即使这看起来好像 if y 只作用于 6
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

//@ 绑定
fn test7() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
