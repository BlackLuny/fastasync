use std::time::Duration;
use std::{sync::Arc, time::Instant};

use dis_core::{Schedular, DummyTaskBehaviour, TaskType};
use tokio::sync::Mutex;

use futures::stream::iter;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let all_client = Arc::new(Mutex::new(Vec::new()));
    for i in 0..1 {
        iter(0..20)
            .for_each_concurrent(0, |_| {
                let all_client_clone = all_client.clone();
                async move {
                    let schedular = Schedular::try_new("127.0.0.1:2181", DummyTaskBehaviour {})
                        .await
                        .unwrap();
                    all_client_clone.lock().await.push(schedular);
                }
            })
            .await;
    }
    tokio::time::sleep(Duration::from_secs(20)).await;
    println!("start");
    if std::env::args().len()>1 {
        let now = Instant::now();
        all_client
            .lock()
            .await
            .first()
            .unwrap()
            .spawn_task(1, TaskType::Spread, Some(19))
            .await
            .unwrap();
            tokio::time::sleep(Duration::from_secs(5)).await;
        
        let mut a = all_client
            .lock()
            .await;

            for x in a.drain(..) {
                x.zk_mng.debug_info().await;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            
        println!("time used: {}", now.elapsed().as_millis());
    }
    drop(all_client);
    tokio::time::sleep(Duration::from_secs(500)).await;
}