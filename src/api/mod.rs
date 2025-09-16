use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

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
