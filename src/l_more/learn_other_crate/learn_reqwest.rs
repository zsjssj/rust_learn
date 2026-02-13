use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::LazyLock;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

pub fn api_test() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(test());
    match res {
        Ok(a) => {
            let aa = a.data.unwrap();
            println!("a:{:#?}", aa)
        }
        Err(_) => println!("错误"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    status: u16,
    msg: String,
    data: Option<serde_json::Value>,
}

async fn test() -> Result<ApiResponse, reqwest::Error> {
    let server_url = "http://10.210.200.109:3000/api/v1/gs-app";
    let url: String = format!("{}/list-projects", server_url); // 构建完整的URL
    // let response = reqwest::get(url).await?.json().await?;
    let response = CLIENT.get(url).send().await?.json().await?;
    println!("response is {}", response);
    println!("json response: {:#?}", serde_json::to_string_pretty(&response).unwrap());
    Ok(ApiResponse {
        status: 200,
        msg: "Success".to_string(),
        data: Some(response),
    })
}

// src-tauri/src/http.rs
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
pub struct HttpService {
    pub client: Client,
    pub base_url: String,
}
impl HttpService {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }
    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let res = self.client.get(url).send().await?;
        Ok(res.json::<T>().await?)
    }
}
