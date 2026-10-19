#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use dotenvy;
use std::env;

#[tokio::main]
pub async fn run() {
    test_1().unwrap();
}
//测试dotenvy的用法
fn test_1() -> Result<(), Box<dyn std::error::Error>> {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let app_path = std::env::current_dir()?;

    let env_file: std::path::PathBuf = match app_env.as_str() {
        "development" => app_path.join(".env.development"),
        "test" => app_path.join(".env.test"),
        "production" => app_path.join(".env.production"),
        other => {
            return Err(format!("未知运行环境：{other}").into());
        }
    };
    println!("env_file: {}", env_file.display());
    dotenvy::from_path(env_file.clone())?;
    // dotenvy::from_filename(env_file.clone())?;

    {
        println!("=== dotenvy 用法演示 ===\n");

        // 1. 最基础用法：加载 .env 文件到当前进程环境变量
        // dotenv() 会查找当前目录及父目录中的 .env 文件
        println!("1. 基础用法 - 加载 .env 文件:");
        match dotenvy::dotenv() {
            Ok(path) => println!("   已加载 .env 文件: {:?}", path),
            Err(e) => println!("   加载失败: {}", e),
        }

        // 2. 读取运行进程已加载的环境变量，
        println!("\n2. 读取环境变量:");
        // 使用标准库读取（dotenv() 已将变量注入进程环境）
        match std::env::var("DATABASE_URL") {
            Ok(val) => println!("   DATABASE_URL = {}", val),
            Err(e) => println!("   DATABASE_URL 未设置: {}", e),
        }
        //尝试读取系统变量：CARGO_HOME，配置文件无这个变量，运行进程变量默认从系统变量与用户变量继承。
        match std::env::var("CARGO_HOME") {
            Ok(val) => println!("   CARGO_HOME = {}", val),
            Err(e) => println!("   CARGO_HOME 未设置: {}", e),
        }

        // 3. dotenvy::var() —— 一步完成加载+读取（推荐）
        println!("\n3. dotenvy::var() - 直接读取（自动加载 .env）:");
        match dotenvy::var("DATABASE_URL") {
            Ok(val) => println!("   DATABASE_URL = {}", val),
            Err(e) => println!("   读取失败: {}", e),
        }

        // 4. dotenv_override() —— 加载 .env 并覆盖已有的系统环境变量
        println!("\n4. dotenv_override() - 覆盖已有环境变量:");
        unsafe {
            std::env::set_var("MY_APP_MODE", "system-value"); // 先设置一个系统环境变量
        }
        println!("   设置前: MY_APP_MODE = {:?}", std::env::var("MY_APP_MODE").ok());
        dotenvy::dotenv_override().ok(); // dotenv_override 会让 .env 中的值优先
        println!("   dotenv_override 后（.env 中若无 MY_APP_MODE，值保持不变）");

        // 5. 遍历 .env【默认】 文件中所有键值对（不注入进程环境）
        println!("\n5. 遍历 .env 文件内容（不修改进程环境）:");
        match dotenvy::dotenv_iter() {
            Ok(iter) => {
                for item in iter {
                    match item {
                        Ok((key, value)) => println!("   {} = {}", key, value),
                        Err(e) => println!("   解析错误: {}", e),
                    }
                }
            }
            Err(e) => println!("   打开 .env 失败: {}", e),
        }

        // 6. 从指定路径加载自定义 .env 文件
        println!("\n6. 从指定路径加载 .env 文件:");
        // let custom_path = std::path::Path::new(env_file);
        let custom_path = env_file.clone();
        match dotenvy::from_path(custom_path.clone()) {
            Ok(()) => println!("   自定义路径加载成功"),
            Err(e) => println!("   加载失败: {}", e),
        }

        // 7. 从指定路径遍历，不注入环境
        println!("\n7. from_path_iter() - 遍历指定文件:");
        match dotenvy::from_path_iter(custom_path) {
            Ok(iter) => {
                let entries: Vec<_> = iter.filter_map(|r| r.ok()).collect();
                println!("   共 {} 个变量", entries.len());
                for (k, v) in &entries {
                    println!("   {} => {}", k, v);
                }
            }
            Err(e) => println!("   失败: {}", e),
        }

        // 8. 实际工程用法：在程序入口处初始化，缺失时 panic
        println!("\n8. 工程实践 - 启动时强制加载，缺少变量则 panic:");
        // dotenvy::dotenv().expect(".env 文件不存在，请先创建");
        // let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL 未配置");
        // 这里仅演示，不实际 panic
        let db_url = dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "未配置".to_string());
        println!("   db_url = {}", db_url);
        println!("\n=== dotenvy 演示完成 ===");
    };

    Ok(())
}
