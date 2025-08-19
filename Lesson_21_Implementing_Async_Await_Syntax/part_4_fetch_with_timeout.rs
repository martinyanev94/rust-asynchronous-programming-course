async fn fetch_with_timeout(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .timeout(std::time::Duration::from_secs(5)) // Set timeout to 5 seconds
        .send()
        .await?;

    let body = response.text().await?;
    Ok(body)
}
