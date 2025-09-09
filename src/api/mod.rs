pub fn api_test() {
    test();
}

struct ApiResponse {
    code: i32,
    msg: String,
    data: Option<serde_json::Value>,
}

fn test() -> Result<ApiResponse, reqwest::Error> {
    // let server_url = "http://10.210.200.109:3000/api/v1";
    // let url = format!("{}/users", server_url); // 构建完整的URL
    // println!("API test run");
    // let client: reqwest::Client = reqwest::Client::new();
    // client
    //     .get(url)

    Ok(ApiResponse {
        code: 200,
        msg: "Success".to_string(),
        data: None,
    })
}
