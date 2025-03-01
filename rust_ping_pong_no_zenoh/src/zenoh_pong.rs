use std::time::Instant;

#[tokio::main]
async fn main() -> zenoh::Result<()> {
    let session = zenoh::open(zenoh::Config::default()).await?;
    let ping_topic = "demo/ping";
    let pong_topic = "demo/pong";

    let pong_subscriber = session.declare_subscriber(ping_topic).await?;
    let pong_publisher = session.declare_publisher(pong_topic).await?;

    let mut inner_count = 0;
    let mut last = Instant::now();

    loop {
        if let Ok(sample) = pong_subscriber.recv_async().await {
            let num = sample.payload().to_bytes()[0];
            pong_publisher.put(num.wrapping_add(1).to_ne_bytes()).await?;
            inner_count += 1;
            if inner_count >= 10_000 {
                println!(
                    "Pong loop frequency: {} Hz",
                    1_000_000 * inner_count / last.elapsed().as_micros()
                );
                inner_count = 0;
                last = Instant::now();
            }
        }
    }
}