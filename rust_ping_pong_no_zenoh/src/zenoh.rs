use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() -> zenoh::Result<()> {
    let session = zenoh::open(zenoh::Config::default()).await?;
    let ping_topic = "demo/ping";
    let pong_topic = "demo/pong";

    // Ping Task
    let ping_session = session.clone();
    let ping = task::spawn(async move {
        let ping_subscriber = ping_session.declare_subscriber(pong_topic).await.unwrap();
        let ping_publisher = ping_session.declare_publisher(ping_topic).await.unwrap();

        let init: u64 = 0;
        ping_publisher.put(init.to_ne_bytes()).await.unwrap();

        loop {
            if let Ok(sample) = ping_subscriber.recv_async().await {
                let num = sample.payload().to_bytes()[0];
                ping_publisher.put(num.wrapping_add(1).to_ne_bytes()).await.unwrap();
            }
        }
    });

    // Pong Task
    let pong_session = session.clone();
    let pong = task::spawn(async move {
        let pong_subscriber = pong_session.declare_subscriber(ping_topic).await.unwrap();
        let pong_publisher = pong_session.declare_publisher(pong_topic).await.unwrap();

        let mut inner_count = 0;
        let mut last = Instant::now();

        loop {
            if let Ok(sample) = pong_subscriber.recv_async().await {
                let num = sample.payload().to_bytes()[0];
                pong_publisher.put(num.wrapping_add(1).to_ne_bytes()).await.unwrap();

                inner_count += 1;
                if inner_count >= 500_000 {
                    println!("Pong loop frequency: {} Hz", 1_000_000 * inner_count / last.elapsed().as_micros());
                    inner_count = 0;
                    last = Instant::now();
                }
            }
        }
    });

    let _ = tokio::join!(ping, pong);
    Ok(())
}