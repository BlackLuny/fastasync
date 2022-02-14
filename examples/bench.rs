use std::time::Duration;
use std::{sync::Arc, time::Instant};

use dis_core::{Schedular, DummyTaskBehaviour, TaskType};
use tokio::sync::Mutex;

use futures::stream::iter;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let all_client = Arc::new(Mutex::new(Vec::new()));
    for i in 0..4 {
        iter(0..100)
            .for_each_concurrent(0, |_| {
                let all_client_clone = all_client.clone();
                async move {
                    let schedular = Schedular::try_new("192.168.3.43:2181", DummyTaskBehaviour {})
                        .await
                        .unwrap();
                    all_client_clone.lock().await.push(schedular);
                }
            })
            .await;
    }
    //tokio::time::sleep(Duration::from_secs(20)).await;
    if std::env::args().len()>1 {
        let now = Instant::now();
        all_client
            .lock()
            .await
            .first()
            .unwrap()
            .spawn_task(1, TaskType::Spread, Some(199))
            .await
            .unwrap();
        println!("time used: {}", now.elapsed().as_millis());
    }

    tokio::time::sleep(Duration::from_secs(500)).await;
}