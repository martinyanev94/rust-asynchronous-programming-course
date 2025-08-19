#[tokio::main]
async fn main() {
    let result = get_future_example().await;
    println!("{}", result);
}
