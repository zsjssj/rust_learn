//! 理解thiserror用法
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("配置文件读取失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("配置解析失败: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("字段缺失: {field}")]
    MissingField { field: String },

    #[error("端口超出范围 (0-65535): {0}")]
    PortOutOfRange(u16),
}

#[derive(Debug, Deserialize)]
struct Config {
    msg: String,
}
fn load_config(path: impl AsRef<std::path::Path>) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?; // io::Error 自动转为 ConfigError::Io
    let config: Config = toml::from_str(&content)?; // 同理
    Ok(config)
}

pub fn run() {
    let path = std::env::current_dir().unwrap().join("Cargo.toml");
    match load_config(path) {
        Err(n) => {
            println!("n 的值：{}", n)
        }
        _ => {}
    };
}
