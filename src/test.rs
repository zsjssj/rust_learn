#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use axum::{
    self, Router,
    extract::{Json, Path, Query, State},
    routing::{delete, get, post, put},
};
use tower_http::services::fs::ServeDir;

use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener; //异步式的TcpListener

#[tokio::main]
pub async fn run() {
    //用户路由
    let user_router: Router = Router::new()
        .route("/user/login", get(|| async { "Login Page" }))
        .route("/user/logout", get(|| async { "Logout Page" }));

    //产品路由
    #[derive(Deserialize)]
    struct DetailParams {
        name: String,
    }

    let product_routes: Router = Router::new()
        .route("/list", get(|| async { "Product List" }))
        .route(
            "/detail/{id}",
            get(|Path(id): Path<u32>| async move { format!("Product Detail for ID: {}", id) }),
        )
        .route(
            "/detail/{id}",
            delete(|Path(id): Path<u32>| async move { format!("delete Product Detail for ID: {}", id) }),
        )
        .route(
            "/detail",
            get(
                |Query(params): Query<DetailParams>| async move { format!("Product Detail for Name: {}", params.name) },
            ),
        )
        .route(
            "/detail",
            post(
                |Json(data): Json<DetailParams>| async move { format!("add Product Detail Post for Name: {}", data.name) },
            ),
        )
        .route(
            "/detail",
            put(|Json(data): Json<DetailParams>| async move {
                format!("update Product Detail Post for Name: {}", data.name)
            }),
        );

    #[derive(Deserialize)]
    struct AppState {
        state: bool,
    }
    let app_state = AppState { state: true };
    let app_routes: Router = Router::new()
        .route(
            "/",
            get(|State(state): State<Arc<AppState>>| async move { format!("app state: {}", state.state) }),
        )
        .with_state(Arc::new(app_state));

    let v1_routes: Router = Router::new()
        .merge(user_router) //平级合并路由
        .nest("/products", product_routes)
        .nest("/app", app_routes);

    let app: Router = Router::new()
        .nest("/api/v1", v1_routes)
        .nest_service("/static", ServeDir::new("assets")) //静态资源路由,返指定文件夹下的传入名称的文件
        .fallback_service(ServeDir::new("assets"))
        .fallback(|| async { "404 Not Found" });

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
