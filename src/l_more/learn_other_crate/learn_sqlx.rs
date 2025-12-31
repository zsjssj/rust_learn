//! Learn more about using the `sqlx` crate for interacting with PostgreSQL databases in Rust.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sqlx::{Pool, Postgres};

#[tokio::main]
pub async fn run() {
    test1().await.unwrap();
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
