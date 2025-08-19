async fn fetch_data_1() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    String::from("Data from endpoint 1")
}

async fn fetch_data_2() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    String::from("Data from endpoint 2")
}

#[tokio::main]
async fn main() {
    let (result1, result2) = tokio::join!(fetch_data_1(), fetch_data_2());
    println!("{} and {}", result1, result2);
}
