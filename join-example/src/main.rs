use tokio::sync::mpsc;
use tokio::time::{self, Duration};

async fn task_one(tx: mpsc::Sender<&'static str>) {
    // Simulate some asynchronous work
    time::sleep(Duration::from_secs(1)).await;
    let _ = tx.send("Task One completed").await;
}

async fn task_two(tx: mpsc::Sender<&'static str>) {
    // Simulate some asynchronous work
    time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send("Task Two completed").await;
}

async fn main_task() {
    // Create a channel for sending messages
    let (tx, mut rx) = mpsc::channel(10);

    // Spawn two asynchronous tasks
    let tx_1 = tx.clone();
    let task_one_handle = tokio::spawn(async move {
        task_one(tx_1).await;
    });
    let tx_2 = tx.clone();
    let task_two_handle = tokio::spawn(async move {
        task_two(tx_2).await;
    });

    // Wait for both tasks to complete using `join`
    let _res = tokio::join!(task_one_handle, task_two_handle);

    // Select between the results of the tasks
    loop {
        tokio::select! {
            Some(message) = rx.recv() => {
                println!("Received message: {}", message);
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                println!("Timeout reached. Exiting.");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    main_task().await;
}
