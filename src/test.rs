#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use super::l_more::calculate_parabola_from_points;
use anyhow::Result;
use std::ops::Add;
use std::sync::LazyLock;
use std::time::Instant;
static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

pub fn run() {
    //记录运行时间
    test3();
}

fn test1() {
    //记录运行时间
    let start = Instant::now();
    let p = calculate_parabola_from_points([(1.0, 2.0), (2.0, 3.0), (3.0, 5.0)]);
    let duration = start.elapsed();
    println!("Time elapsed in run() is: {:?}", duration);
    println!("test run completed {:?}", p);
}

fn test3() {
    let s1: String = String::from("hello");
    let joinhandler = std::thread::Builder::new()
        .name("worker".into())
        .spawn(|| {
            let s2 = s1 + " world";
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("test3 completed {:?}", s2);
        })
        .unwrap();
    let info = joinhandler.thread();
    println!("thread info: {:?}", info);
    joinhandler.join().unwrap();
}
