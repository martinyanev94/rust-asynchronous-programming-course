async fn fetch_with_error() -> Result<String, String> {
    // Simulating an error condition
    Err(String::from("Network error"))
}

#[tokio::main]
async fn main() {
    match fetch_with_error().await {
        Ok(data) => println!("Fetched: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
