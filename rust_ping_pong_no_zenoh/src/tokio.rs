use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (ping_tx, mut ping_rx) = mpsc::channel::<u8>(1000);
    let (pong_tx, mut pong_rx) = mpsc::channel::<u8>(1000);

    let ping = tokio::spawn(async move {
        let mut num: u8 = 0;
        ping_tx.send(num).await.unwrap();
        loop {
            if let Some(received) = pong_rx.recv().await {
                num = received.wrapping_add(1);
                ping_tx.send(num).await.unwrap();
            }
        }
    });

    let pong = tokio::spawn(async move {
        let mut count = 0;
        let mut last = Instant::now();
        loop {
            if let Some(received) = ping_rx.recv().await {
                pong_tx.send(received.wrapping_add(1)).await.unwrap();
                count += 1;
                if count >= 500_000 {
                    println!(
                        "Pong loop frequency: {} Hz",
                        1_000_000 * count / last.elapsed().as_micros()
                    );
                    count = 0;
                    last = Instant::now();
                }
            }
        }
    });

    let _ = tokio::join!(ping, pong);
}