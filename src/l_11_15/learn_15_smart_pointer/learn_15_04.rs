/*
  Rc<T> 引用计数智能指针,【单线程】
  参考：https://chatgpt.com/c/68da4cbe-a744-8323-9868-e3a05cc71016
*/
#![allow(dead_code)]
#![allow(unused_variables)]
use std::rc::Rc;

pub fn run() {
    test2();
}

//1. 使用 Rc<T> 共享数据
fn test1() {
    use List::{Cons, Nil};
    use RcList::{Cons as RcCons, Nil as RcNil};
    let a: List = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b: List = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));    // 这行代码会导致编译错误，因为 a 已经被移动到 b 中，不能再被使用

    let a1: Rc<RcList> = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    let b: RcList = RcCons(3, Rc::clone(&a1)); // 通过克隆 Rc 指针来共享所有权
    let c: RcList = RcCons(4, a1.clone()); // 通过克隆 Rc 指针来共享所有权
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

//2. 克隆 Rc<T> 会增加引用计数，而不是复制数据。【Rc::strong_count(&self)】
fn test2() {
    use RcList::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, a.clone());
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
