#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use super::learn_more::learn_algorithm::Solution;
use anyhow::Ok;
use rand::Rng;
use std::collections::HashSet;
use std::pin::{Pin, pin};

pub fn run() {
    let mut s1 = String::from("hello");
    let a1 = 10;
    let a2 = 20;
    let a3 = a2.max(a1);
    println!("max of {} and {} is {}", a1, a2, a3);
}
