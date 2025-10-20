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
    // test3().await;
    // test4().await;
    // test42().await;
    // test43().await;
    test44().await;
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
    format!("Hello, {name}!")
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

use axum::extract::Extension;
async fn test42() {
    let app_state = AppState {
        message: String::from("Hello from Extension!"),
    }; // 应用状态，可以通过 State 提取器访问
    let app: Router = Router::new().route("/state", get(handler2)).layer(Extension(Arc::new(app_state)));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn handler2(Extension(state): Extension<Arc<AppState>>) -> String {
    format!("State message: {}", state.message)
}

//Using closure captures
use axum::routing::post;
async fn test43() {
    let shared_state = Arc::new(AppState {
        message: "Hello from closure captures!".to_string(),
    });
    let app: Router = Router::new()
        .route(
            "/users",
            post({
                let shared_state: Arc<AppState> = shared_state.clone();
                move |body| create_user(body, shared_state)
            }),
        )
        .route(
            "/users/{id}",
            get({
                let shared_state: Arc<AppState> = Arc::clone(&shared_state);
                move |path| get_user(path, shared_state)
            }),
        );
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) -> String {
    format!("User message: {}", state.message)
}
async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) -> String {
    format!("Created user: {} (age {}) with message: {}", payload.name, payload.age, state.message)
}
#[derive(Deserialize)]
struct CreateUserPayload {
    name: String,
    age: u8,
}

//Using task-local variables
use axum::{
    // Router,
    extract::Request,
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    // routing::get,
};
use tokio::task_local;
async fn test44() {
    let app: Router = Router::new().route("/", get(handler4)).route_layer(middleware::from_fn(auth));
    let app = Router::new().nest("/api/v1", app); //nest 嵌套路由（推荐用于 API 子路由）
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct CurrentUser {
    name: String,
}
task_local! {
    pub static USER: CurrentUser;
}
async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|header| header.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if let Some(current_user) = authorize_current_user(auth_header).await {
        println!("✅ 用户 {} 已通过认证", current_user.name);
        // State is setup here in the middleware
        println!("Authenticated user: {}", current_user.name);
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser { name: auth_token.to_string() })
}

struct UserResponse;
impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        // State is accessed here in the IntoResponse implementation
        let current_user = USER.with(|u| u.clone());
        (StatusCode::OK, current_user.name).into_response()
    }
}
async fn handler4() -> UserResponse {
    UserResponse
}
