#![allow(unused)]
// use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

pub fn run() {
    let a = Rc::new(RefCell::new(5));
    let b = Rc::clone(&a);
    *b.borrow_mut() += 1;
    println!("a = {}", a.borrow());
    println!("b = {}", b.borrow());

    let c = Rc::new(Cell::new(10));
    let d = Rc::clone(&c);
    d.set(20);
    println!("c = {}", c.get());
    println!("d = {}", d.get());
}
