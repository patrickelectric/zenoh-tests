use std::net::UdpSocket;
use std::str;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:1234")?;
    let mut buf = [0u8; 1024];
    let mut count = 0;


    let mut last_response    = Instant::now();
    let mut inner_count = 0;

    loop {
        let (size, source) = socket.recv_from(&mut buf)?;
        if let Ok(text) = str::from_utf8(&buf[..size]) {
            if let Ok(num) = text.trim().parse::<u64>() {
                count = num + 1;
                socket.send_to(count.to_string().as_bytes(), &source)?;
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
