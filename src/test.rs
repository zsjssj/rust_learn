#![allow(unused)]
use anyhow::{Context, Result};
use std::fs;

fn read_config() -> Result<String> {
    // 任何 io::Error 会被 ? 自动转换为 anyhow::Error
    let s = fs::read_to_string("config.toml").context("读取配置文件失败：config.toml")?;
    Ok(s)
}

pub fn test1() -> Result<()> {
    let config = read_config()?;
    println!("配置内容：\n{}", config);
    Ok(())
}

pub fn run() {
    // test1().unwrap();
    match test1() {
        Ok(_) => println!("配置文件读取成功"),
        Err(e) => println!("发生错误：{}", e),
    }
}
