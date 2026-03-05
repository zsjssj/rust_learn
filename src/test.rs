#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use super::l_more::calculate_parabola_from_points;
use anyhow::Result;
use core::time;
use std::ops::Add;
use std::sync::LazyLock;
use std::thread::JoinHandle;
use std::time::Instant;
static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

pub fn run() {
    //记录运行时间
    // test3();
    // test4();
    let v: Vec<u32> = vec![0; 100000]
        .iter()
        .map(|_| rand::random::<u32>())
        .collect::<Vec<_>>();
    let mut v1 = v.clone();
    let mut v2 = v.clone();
    loga(&mut v1);
    logb(&mut v2);
}

fn test1() {
    //记录运行时间
    let start = Instant::now();
    let p = calculate_parabola_from_points([(1.0, 2.0), (2.0, 3.0), (3.0, 5.0)]);
    let duration = start.elapsed();
    println!("Time elapsed in run() is: {:?}", duration);
    println!("test run completed {:?}", p);
}

fn loga(vec: &mut Vec<u32>) {
    use rayon::prelude::*;
    let timer = Instant::now();
    vec.par_sort();
    let duration = timer.elapsed();
    println!("Time elapsed in loga() is: {:?}", duration);
}
fn logb(vec: &mut Vec<u32>) {
    let timer = Instant::now();
    vec.sort();
    let duration = timer.elapsed();
    println!("Time elapsed in logb() is: {:?}", duration);
}
