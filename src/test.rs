#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::Result;
use dotenvy;
use std::env;

#[tokio::main]
pub async fn run() {
    test_1().unwrap();
}
//测试dotenvy的用法
fn test_1() -> Result<()> {
    let s1 = "ssje hello zsj abcd".to_string();
    // let a = s1;
    Ok(())
}
