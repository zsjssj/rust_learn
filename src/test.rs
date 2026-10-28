#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::Result;
use dotenvy;
use std::{env, marker::PhantomPinned, num, pin::Pin};

#[tokio::main]
pub async fn run() {
    test_1();
}
//测试dotenvy的用法
fn test_1() -> std::result::Result<(), String> {
    let ort_library_name = match std::env::consts::OS {
        "windows" => "onnxruntime.dll",
        "linux" => "libonnxruntime.so.1.29.0",
        other => return Err(format!("unsupported OS for ONNX Runtime: {other}").into()),
    };
    println!("ort_library_name is {}", ort_library_name);

    use std::boxed::Box;
    let mut number = Box::new(100);
    *number = 99;
    println!("number: {}", number);

    Ok(())
}
