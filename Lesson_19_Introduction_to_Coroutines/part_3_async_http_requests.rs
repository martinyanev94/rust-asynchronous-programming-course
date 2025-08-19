async fn async_main() {
    println!("Coroutine starting.");
    let response1 = Http::get("/some_endpoint").await;
    println!("Response received: {}", response1);

    let response2 = Http::get("/another_endpoint").await;
    println!("Second response received: {}", response2);
}
