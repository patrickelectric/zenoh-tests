use tokio::net::UdpSocket;
use std::time::Instant;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Server Task
    let server = tokio::spawn(async {
        let socket = UdpSocket::bind("127.0.0.1:1234").await.unwrap();
        let mut buf = [0u8; 1024];
        loop {
            let (size, src) = socket.recv_from(&mut buf).await.unwrap();
            if size >= 8 {
                let num = u64::from_ne_bytes(buf[..8].try_into().unwrap());
                let new_num = num + 1;
                let _ = socket.send_to(&new_num.to_ne_bytes(), &src).await;
            }
        }
    });

    // Client Task
    let client = tokio::spawn(async {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        socket.connect("127.0.0.1:1234").await.unwrap();
        let mut buf = [0u8; 1024];
        let mut inner_count = 0;
        let mut last = Instant::now();
        let init: u64 = 0;
        let _ = socket.send(&init.to_ne_bytes()).await;
        loop {
            let size = socket.recv(&mut buf).await.unwrap();
            if size >= 8 {
                let num = u64::from_ne_bytes(buf[..8].try_into().unwrap());
                let _ = socket.send(&num.wrapping_add(1).to_ne_bytes()).await;
            }
            inner_count += 1;
            if inner_count >= 10_000 {
                println!(
                    "Client loop frequency: {} Hz",
                    1_000_000 * inner_count / last.elapsed().as_micros()
                );
                inner_count = 0;
                last = Instant::now();
            }
        }
    });

    let _ = tokio::join!(server, client);
    Ok(())
}
