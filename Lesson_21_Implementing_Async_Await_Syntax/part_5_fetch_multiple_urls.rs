#[tokio::main]
async fn main() {
    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/post",
        "https://httpbin.org/put"
    ];

    let futures: Vec<_> = urls.into_iter()
        .map(|url| fetch_url(url))
        .collect();

    let results = futures::future::join_all(futures).await;

    for result in results {
        match result {
            Ok(body) => println!("Response Body: {}", body),
            Err(err) => eprintln!("Error fetching URL: {}", err),
        }
    }
}
