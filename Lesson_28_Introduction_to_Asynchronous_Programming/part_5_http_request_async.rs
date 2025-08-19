use reqwest;

#[tokio::main]
async fn main() {
    let response = reqwest::get("http://httpbin.org/get").await.unwrap();
    let body = response.text().await.unwrap();
    println!("Response Body: {}", body);
}
