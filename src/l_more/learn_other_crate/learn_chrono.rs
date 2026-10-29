#![allow(unused_imports)]
use chrono::{Duration, Utc};
pub fn run() {
    let t1 = Utc::now().naive_local();
    println!("t1: {}", t1);
    let t2 = t1 - Duration::minutes(15);
    println!("t2: {}", t2);
}
