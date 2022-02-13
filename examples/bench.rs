#[tokio::main]
async fn main() {
    let all_client = Arc::new(Mutex::new(Vec::new()));
    for i in 0..20 {
        iter(0..100)
            .for_each_concurrent(0, |_| {
                let all_client_clone = all_client.clone();
                async move {
                    let schedular = Schedular::try_new(ZK_ADDR, DummyTaskBehaviour {})
                        .await
                        .unwrap();
                    all_client_clone.lock().await.push(schedular);
                }
            })
            .await;
    }

    let now = Instant::now();
    all_client
        .lock()
        .await
        .first()
        .unwrap()
        .spawn_task(1, TaskType::Spread, Some(500))
        .await
        .unwrap();
    println!("time used: {}", now.elapsed().as_millis());
    //tokio::time::sleep(Duration::from_secs(50)).await;
}