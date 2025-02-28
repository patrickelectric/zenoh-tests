use std::net::UdpSocket;
use std::str;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    // Bind to an ephemeral port for the client.
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = "127.0.0.1:1234";
    let mut buf = [0u8; 1024];
    let mut count = 0;

    let mut inner_count = 0;
    let mut last_response = Instant::now();

    socket.send_to(count.to_string().as_bytes(), server_addr)?;
    loop {
        let (size, _) = socket.recv_from(&mut buf)?;
        if let Ok(response) = str::from_utf8(&buf[..size]) {
            if let Ok(num) = response.trim().parse::<u64>() {
                count = num + 1;
                socket.send_to(count.to_string().as_bytes(), &server_addr)?;
            }
        }

        inner_count += 1;
        if last_response.elapsed().as_secs() >= 1 {
            println!("Loop frequency: {} Hz", inner_count / last_response.elapsed().as_secs());
            inner_count = 0;
            last_response = Instant::now();
        }
    }
}
