#![allow(unused)]
// use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

pub fn run() {
    test1();
}
fn test1() {
    async_fn();
}
fn async_fn() {
    let f1 = async {
        trpl::sleep(Duration::from_secs(1)).await;
        println!("async fn done");
    };
    let f2 = async {
        trpl::sleep(Duration::from_secs(1)).await;
        println!("async fn2 done");
    };
    println!("test1 start");
    // 并行执行两个任务
    trpl::join(f1, f2);
    println!("test1 done");
}
