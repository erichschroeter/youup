
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

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
    let is_up = is_server_up(addr).await;
    if is_up {
        println!("Server {addr} is up");
    } else {
        println!("Server {addr} is not up");
    }
}
