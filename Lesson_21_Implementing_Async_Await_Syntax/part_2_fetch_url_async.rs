#[tokio::main]
async fn main() {
    let url = "https://httpbin.org/get";
    match fetch_url(url).await {
        Ok(body) => println!("Response Body: {}", body),
        Err(err) => eprintln!("Error fetching URL: {}", err),
    }
}
