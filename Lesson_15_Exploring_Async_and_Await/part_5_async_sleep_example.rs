#[tokio::main]
async fn main() {
    let future = async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        "Data from async block"
    };

    let result = future.await;
    println!("{}", result);
}
