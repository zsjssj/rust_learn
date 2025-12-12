#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use core::str;

use serde::{Deserialize, Serialize};

pub fn run() {
    let p1 = Person {
        base: Base {
            name: String::from("Alice"),
            age: 30,
        },
        score: 95,
    };
    // 序列化为 JSON 字符串
    let json_str = serde_json::to_string(&p1).unwrap();
    println!("Serialized JSON: {}", json_str);
    // 反序列化回结构体
    let p2: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized Person: {:?}", p2);
    //创建一个json数据
    let json_data = r#"
    {
        "name": "Bob",
        "age": 25,
        "score": 88,
        "sex":1
    }"#;
    let p3: Person = serde_json::from_str(json_data).unwrap();
    println!("Deserialized Person with extra field: {:?}", p3);
}

#[derive(Deserialize, Serialize, Debug)]
struct Base {
    pub name: String,
    pub age: u8,
}

#[derive(Deserialize, Serialize, Debug)]
struct Person {
    #[serde(flatten)]
    pub base: Base,
    pub score: u8,
}
