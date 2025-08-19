#[tokio::main]
async fn main() -> io::Result<()> {
    fetch_data().await?;
    Ok(())
}
