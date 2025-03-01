#[tokio::main]
async fn main() -> zenoh::Result<()> {
    let session = zenoh::open(zenoh::Config::default()).await?;
    let ping_topic = "demo/ping";
    let pong_topic = "demo/pong";

    let ping_subscriber = session.declare_subscriber(pong_topic).await?;
    let ping_publisher = session.declare_publisher(ping_topic).await?;

    let init: u64 = 0;
    ping_publisher.put(init.to_ne_bytes()).await?;

    loop {
        if let Ok(sample) = ping_subscriber.recv_async().await {
            let num = sample.payload().to_bytes()[0];
            ping_publisher.put(num.wrapping_add(1).to_ne_bytes()).await?;
        }
    }
}