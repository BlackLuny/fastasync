use std::time::{Duration, Instant};

use dis_core::{DummyTaskBehaviour, Schedular, TaskType};
#[tokio::main]
async fn main() {
    if std::env::args().len() == 1 {
        let schedular = Schedular::try_new("127.0.0.1:2181", DummyTaskBehaviour {})
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(600)).await;
    } else {
        let schedular = Schedular::try_new("127.0.0.1:2181", DummyTaskBehaviour {})
            .await
            .unwrap();
        println!("start");
        let start = Instant::now();
        schedular.spawn_task(10, TaskType::Spread, Some(1)).await.unwrap();
        println!("time used: {}", start.elapsed().as_millis());
    }

}
