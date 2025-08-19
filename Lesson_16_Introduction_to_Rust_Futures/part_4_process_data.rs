use tokio::task;

async fn process_data(data: Vec<u8>) {
    let analysis_future = task::spawn_blocking(move || {
        // CPU-intensive processing happens here
        data.iter().map(|x| x * 2).collect::<Vec<u8>>()
    });

    let result = analysis_future.await.unwrap();
    println!("Processed data: {:?}", result);
}
