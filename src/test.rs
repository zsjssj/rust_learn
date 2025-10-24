#![allow(unused)]
// use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

use rand::distr::uniform::SampleBorrow;

pub fn run() {
    test1();
}
fn test1() {
    let c1: Rc<Cell<i32>> = Rc::new(Cell::new(5));
    let c2: Rc<Cell<i32>> = Rc::clone(&c1);
    c2.set(10);
    println!("c1 = {}, c2 = {}", c1.get(), c2.get());

    let c3: Rc<RefCell<String>> = Rc::new(RefCell::new("ssje".to_string()));
    let c4: Rc<RefCell<String>> = Rc::clone(&c3);
    c4.borrow_mut().insert_str(0, "hello ");
    println!("c3 = {}, c4 = {}", c3.borrow(), c4.borrow());
    //c4的栈地址，堆区地址
    println!("c4 address = {:p}", c4);
    println!("c4 heap address = {:p}", c4.borrow().as_ptr());
}
