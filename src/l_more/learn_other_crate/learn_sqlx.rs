//! Learn more about using the `sqlx` crate for interacting with PostgreSQL databases in Rust.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sqlx::{
    Pool, Postgres,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::fs::{File, create_dir_all};

#[tokio::main]
pub async fn run() {
    // test1().await.unwrap();
    test2().await.unwrap();
}
#[derive(sqlx::FromRow, Debug)]
struct User {
    id: i32,
    name: String,
}

async fn test1() -> Result<(), sqlx::Error> {
    let pool = Pool::<Postgres>::connect("postgres://postgres:521421@localhost:5432/postgres").await?;
    let user: User = sqlx::query_as("SELECT id, name FROM role WHERE id = $1")
        .bind(2)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .unwrap();
    println!("找到的数据: {:?}", user);
    Ok(())
}

//初始化一个数据库
async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = "postgres://postgres:521421@localhost:5432/postgres";
    let pool = Pool::<Postgres>::connect(database_url).await?;
    Ok(pool)
}

async fn test2() -> Result<(), sqlx::Error> {
    // 连接（不存在就创建）。注意：filename 只需要文件路径，不要带 sqlite:// 前缀
    let db_path = format!("{}/my.db", env!("CARGO_MANIFEST_DIR"));
    println!("db_path = {}", db_path);

    let options = SqliteConnectOptions::new().filename(&db_path).create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    // 建表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    // 每次运行先清空，避免重复插入导致数据累积
    sqlx::query("DELETE FROM users").execute(&pool).await?;

    // 插入数据：execute 返回结果可以拿到受影响行数与自增主键
    let insert_result = sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("Alice")
        .execute(&pool)
        .await?;
    println!(
        "插入成功: 影响行数 = {}, 最后插入的 id = {}",
        insert_result.rows_affected(),
        insert_result.last_insert_rowid()
    );

    // 批量插入更多数据
    for name in ["Bob", "Carol", "Dave"] {
        sqlx::query("INSERT INTO users (name) VALUES (?)")
            .bind(name)
            .execute(&pool)
            .await?;
    }

    // 查询单条数据（按 id）
    let alice: User = sqlx::query_as("SELECT id, name FROM users WHERE name = ?")
        .bind("Alice")
        .fetch_one(&pool)
        .await?;
    println!("查询单条: {:?}", alice);

    // 更新数据
    let update_result = sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind("Alice Updated")
        .bind(alice.id)
        .execute(&pool)
        .await?;
    println!("更新成功: 影响行数 = {}", update_result.rows_affected());

    // 查询全部数据
    let users: Vec<User> = sqlx::query_as("SELECT id, name FROM users ORDER BY id")
        .fetch_all(&pool)
        .await?;
    println!("查询全部，共 {} 条:", users.len());
    for user in &users {
        println!("  {:?}", user);
    }

    // 删除一条数据
    let delete_result = sqlx::query("DELETE FROM users WHERE name = ?")
        .bind("Bob")
        .execute(&pool)
        .await?;
    println!("删除成功: 影响行数 = {}", delete_result.rows_affected());

    // 统计剩余数量
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;
    println!("剩余数据总数: {}", count);

    // 关闭连接池
    pool.close().await;

    Ok(())
}
