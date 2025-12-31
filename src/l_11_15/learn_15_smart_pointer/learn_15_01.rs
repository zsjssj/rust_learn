/*
  Box<T>：指向堆上的数据
  参考:  https://kaisery.github.io/trpl-zh-cn/ch15-01-box.html
*/
#![allow(unused_variables)]
#![allow(dead_code)]

pub fn run() {
    test1();
}
fn test1() {
    let b = Box::new(5);
    println!("*b = {}", *b);
    println!("b = {b}");
}

// 递归类型
// enum List {   //代码无法编译，因为编译器无法知道List类型的大小
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn test2() {
    use List::{Cons, Nil};
    // let list = Cons(1, Cons(2, Cons(3, Nil)));  //代码无法编译
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
