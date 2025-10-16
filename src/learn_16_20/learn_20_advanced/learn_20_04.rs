//!高级函数与闭包
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-04-advanced-functions-and-closures.html

#![allow(dead_code)]
#![allow(unused_variables)]
pub fn run() {
    test1();
    // test2();
}
//函数指针
/*
  函数会被强制转换为 fn 类型（小写的 f），不要与闭包 trait 的 Fn 相混淆。
  fn 被称为 函数指针（function pointer）
*/
fn test1() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings: {:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_statuses: {:?}", list_of_statuses);
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

//返回闭包
fn test2() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

/*
  错误信息告诉我们每当返回一个 impl Trait Rust 会创建一个独特的不透明类型（opaque type），这是一个无法看清 Rust 为我们构建了什么细节的类型
  fn returns_closure() -> impl Fn(i32) -> i32 {
      |x| x + 1
  }
  fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
      move |x| x + init
  }
*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
