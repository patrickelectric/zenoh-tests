use tokio::net::UnixDatagram;
use tempfile::tempdir;
use std::time::Instant;
use std::convert::TryInto;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let tmp = tempdir()?;
    let server_path = tmp.path().join("server.sock");
    let client_path = tmp.path().join("client.sock");

    let server_socket = UnixDatagram::bind(&server_path)?;
    let client_socket = UnixDatagram::bind(&client_path)?;

    // Server task
    let server = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let (size, addr) = server_socket.recv_from(&mut buf).await.unwrap();
            if size >= 8 {
                let num = u64::from_ne_bytes(buf[..8].try_into().unwrap());
                let new_num = num + 1;
                let _ = server_socket.send_to(&new_num.to_ne_bytes(), addr.as_pathname().unwrap()).await;
            }
        }
    });

    // Client task
    let client = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        let mut inner_count = 0;
        let mut last = Instant::now();
        let init: u64 = 0;
        let _ = client_socket.send_to(&init.to_ne_bytes(), &server_path).await;
        loop {
            let (size, _) = client_socket.recv_from(&mut buf).await.unwrap();
            if size >= 8 {
                let num = u64::from_ne_bytes(buf[..8].try_into().unwrap());
                let _ = client_socket.send_to(&num.wrapping_add(1).to_ne_bytes(), &server_path).await;
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

    tokio::join!(server, client);
    Ok(())
}
