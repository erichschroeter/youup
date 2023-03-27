
use std::net::SocketAddr;
use surge_ping::IcmpPacket;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

async fn is_ping(addr: SocketAddr) -> bool {
    // let payload = [0];
    match surge_ping::ping("127.0.0.1".parse().unwrap(), &[0]).await {
        Ok((IcmpPacket::V4(packet), duration)) => {
            println!(
                "{} bytes from {}: icmp_seq={} ttl={:?} time={:.2?}",
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_ttl(),
                duration
            );
            return true;
        }
        Ok(_) => unreachable!(),
        Err(e) => println!("{:?}", e),
    };
    false
}

async fn is_server_up(addr: SocketAddr) -> bool {
    let mut retries = 5; // number of retries
    loop {
        match TcpStream::connect(&addr).await {
            Ok(_) => {
                return true; // server is up
            }
            Err(_) => {
                retries -= 1;
                if retries == 0 {
                    return false; // server is not up after retries
                }
            }
        }
        sleep(Duration::from_secs(1)).await; // wait for 1 second before next retry
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap(); // replace with your server address
    // let is_up = is_server_up(addr).await;
    let is_up = is_ping(addr).await;
    if is_up {
        println!("Server {addr} is up");
    } else {
        println!("Server {addr} is not up");
    }
}
