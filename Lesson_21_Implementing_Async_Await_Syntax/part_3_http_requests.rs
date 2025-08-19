async fn post_data(url: &str, json: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .body(json.to_string())
        .send()
        .await?;

    let body = response.text().await?;
    Ok(body)
}
#[tokio::main]
async fn main() {
    let get_url = "https://httpbin.org/get";
    if let Ok(body) = fetch_url(get_url).await {
        println!("GET Response Body: {}", body);
    }

    let post_url = "https://httpbin.org/post";
    let json_data = r#"{"name": "John", "age": 30}"#;
    match post_data(post_url, json_data).await {
        Ok(body) => println!("POST Response Body: {}", body),
        Err(err) => eprintln!("Error posting data: {}", err),
    }
}
