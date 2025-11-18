#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub fn run() {
    let a = String::from("hello");
    let a1 = &a;
    let &a2 = &a;
    println!("a={}, a1={}, a2={}", a, a1, a2);
}
