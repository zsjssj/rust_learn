//服务端Web框架
#![allow(dead_code)]
#![allow(unused_variables)]

use axum::{self, Router, routing::get};
use serde::Deserialize;
use tokio;

#[tokio::main]
pub async fn run() {
    // test1().await;
    // test2().await;
    test3().await;
}

async fn test1() {
    // 在后台异步任务中启动 axum 服务：tokio::spawn 会创建一个新的异步任务并立即返回，不会阻塞当前线程
    tokio::spawn(async {
        // 创建一个简单的 Router：当收到 GET / 请求时返回静态字符串
        // 这里显式标注类型为 Router，便于阅读和类型推断检查
        let app: Router = Router::new()
            .route("/", get(root)) // get 处理器返回一个异步块，其结果将作为响应体
            .route("/foo", get(get_foo).post(post_foo))
            .route("/foo/bar", get(foo_bar));

        // 绑定 TCP 监听器到本地地址 127.0.0.1:3000
        // bind 是异步操作，需要 await；如果绑定失败（例如端口被占用），unwrap 会使程序 panic
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

        // 使用 axum::serve 启动服务器，接收 listener 上的连接并用 app 处理请求
        // serve 返回一个 Future，因此需要 await；同样使用 unwrap 简化错误处理（遇到错误会 panic）
        axum::serve(listener, app).await.unwrap();
    })
    .await
    .unwrap(); // 等待后台任务完成（实际上服务器会一直运行，除非遇到错误或手动停止）
}
async fn root() -> String {
    "Hello, World!".to_string()
}
async fn get_foo() -> String {
    "Hello, Foo!".to_string()
}
async fn post_foo() -> String {
    "Hello, Foo! Post".to_string()
}
async fn foo_bar() -> String {
    "Hello, Foo Bar!".to_string()
}

use axum::extract::{Json, Path, Query, State};
use std::sync::Arc;
async fn test2() {
    let app: Router = Router::new()
        .route("/hello/{name}", get(hello_handler)) // 使用 {name} 作为路径参数
        .route("/query", get(query_handler)) // Query 提取器处理 /query?q=xxx
        .route("/json", get(json_handler)); // Json 提取器处理 JSON 请求体

    println!("🚀 Server running at http://127.0.0.1:3000");
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Clone)]
struct AppState {
    message: String,
}
#[derive(Deserialize)]
struct Params {
    q: String,
}
#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}
// 1️⃣ Path 提取器：从路径中取参数
async fn hello_handler(Path(name): Path<String>) -> String {
    format!("Hello, {name}! {}")
}
// 2️⃣ Query 提取器：从 ?q=xxx 获取查询参数
async fn query_handler(Query(params): Query<Params>) -> String {
    format!("You searched for: {}", params.q)
}
// 3️⃣ Json 提取器：解析 JSON 请求体
async fn json_handler(Json(user): Json<User>) -> String {
    format!("Hello, {}, age is {}!", user.name, user.age)
}

//
use rand::Rng;
use serde_json::{Value, json};
async fn test3() {
    let app: Router = Router::new().route("/plain_text", get(plain_text)).route("/json", get(json));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn plain_text() -> &'static str {
    "foo"
}
async fn json() -> Json<Value> {
    let mut rng = rand::rng();
    let n: i32 = rng.random_range(10..=200);
    Json(json!({ "data": n }))
}

//Using the State extractor
async fn test4() {
    let app_state = AppState { message: String::from("Hello from state!") }; // 应用状态，可以通过 State 提取器访问
    let app: Router = Router::new().route("/state", get(handler)).with_state(Arc::new(app_state));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn handler(State(state): State<Arc<AppState>>) -> String {
    format!("State message: {}", state.message)
}
