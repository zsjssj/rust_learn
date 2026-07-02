#![allow(unused)]
use std::collections::HashMap;

pub fn test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    _test3();
}
fn _test1() {
    let teams = [String::from("Blue"), String::from("Yellow")];
    let initial_scores = [10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for i in &scores {
        println!("{}: {}", i.0, i.1);
    }
}

// 多次插入数据，会覆盖之前的值
fn _test2() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn _test3() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
