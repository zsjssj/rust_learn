//! 共享状态的并发
// 参考：https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    // test1();
    test2();
}

/// 使用互斥器实现同一时刻只允许一个线程访问数据
/// Mutex<T>的 API
fn test1() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");
}

/// 在多个线程间共享 Mutex<T>
/// 原子引用计数 Arc<T>
fn test2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
