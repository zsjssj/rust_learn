#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use core::sync;

#[tokio::main]
pub async fn run() {
    let a = test_1(true);
    let b = test_2(true);
    println!("test_1: {}, test_2: {}", a, b);
}

fn test_1(condition: bool) -> u32 {
    use std::sync::Arc;
    let op1 = Some(Arc::new(1));
    let op2 = op1.clone();
    let op3 = Arc::clone(op1.as_ref().unwrap());
    let op4 = Arc::downgrade(op1.as_ref().unwrap());
    let count = Arc::strong_count(op1.as_ref().unwrap());
    println!("op1: {:?}, op2: {:?}, count: {}", op1, op2, count);
    drop(op3);
    let count = Arc::strong_count(op1.as_ref().unwrap());
    println!("op1: {:?}, op2: {:?}, count: {}", op1, op2, count);
    if condition { 1 } else { 2 }
}
fn test_2(condition: bool) -> u32 {
    match condition {
        true => 1,
        false => 2,
    }
}
