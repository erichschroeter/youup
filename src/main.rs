
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::path::{PathBuf, Path};
use async_trait::async_trait;
use clap::__macro_refs::once_cell::sync::Lazy;
use clap::{value_parser, Arg, command};
use config::Config;
use mockall::mock;
use surge_ping::IcmpPacket;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

static DEFAULT_CONFIG_FILE: Lazy<PathBuf> = Lazy::new(||{dirs::home_dir().unwrap().join(".config").join("youup").join("default.toml")});


#[derive(Debug, Clone, PartialEq)]
pub enum PingError {
    IcmpError,
    WebSocketError,
}

#[async_trait]
pub trait Pingable {
    async fn ping(&self) -> Result<bool, PingError>;
    fn cache(&self) -> Result<bool, PingError>;
}

pub struct IcmpDest {
    addr: SocketAddr,
    cache: Result<bool, PingError>,
}

impl IcmpDest {
    pub fn new(addr: SocketAddr) -> Self {
        IcmpDest {
            addr,
            cache: Ok(false),
        }
    }
}

impl Default for IcmpDest {
    fn default() -> Self {
        let addr = "127.0.0.1:8080".parse().unwrap();
        IcmpDest::new(addr)
    }
}

#[async_trait]
impl Pingable for IcmpDest {
    async fn ping(&self) -> Result<bool, PingError> {
        Err(PingError::IcmpError)
    }

    fn cache(&self) -> Result<bool, PingError> {
        self.cache.clone()
    }
}
    
mock! {
    pub MockIcmpDest {}

    #[async_trait]
    impl Pingable for MockIcmpDest {
        async fn ping(&self) -> Result<bool, PingError> {
            Err(PingError::IcmpError)
        }

        fn cache(&self) -> Result<bool, PingError> {
            Err(PingError::IcmpError)
        }
    }
}

pub struct WebSocketDest;

// async fn is_ping(addr: SocketAddr) -> bool {
//     // let payload = [0];
//     match surge_ping::ping("127.0.0.1".parse().unwrap(), &[0]).await {
//         Ok((IcmpPacket::V4(packet), duration)) => {
//             println!(
//                 "{} bytes from {}: icmp_seq={} ttl={:?} time={:.2?}",
//                 packet.get_size(),
//                 packet.get_source(),
//                 packet.get_sequence(),
//                 packet.get_ttl(),
//                 duration
//             );
//             return true;
//         }
//         Ok(_) => unreachable!(),
//         Err(e) => println!("{:?}", e),
//     };
//     false
// }

// async fn is_server_up(addr: SocketAddr) -> bool {
//     let mut retries = 5; // number of retries
//     loop {
//         match TcpStream::connect(&addr).await {
//             Ok(_) => {
//                 return true; // server is up
//             }
//             Err(_) => {
//                 retries -= 1;
//                 if retries == 0 {
//                     return false; // server is not up after retries
//                 }
//             }
//         }
//         sleep(Duration::from_secs(1)).await; // wait for 1 second before next retry
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    mod icmp_dest {
        use tokio_test::block_on;

        use super::*;

        #[test]
        fn ping_returns_error() {
            let mut pinger = MockMockIcmpDest::new();
            pinger.expect_ping().returning(|| {
                Err(PingError::IcmpError)
            });
            // let pinger = IcmpDest::default();
            let res = block_on(pinger.ping());
            assert_eq!(PingError::IcmpError, res.err().unwrap());
        }

        #[test]
        fn cache_returns_most_recent_result() {
            let mut pinger = MockMockIcmpDest::new();
            pinger.expect_ping().returning(|| {
                Ok(true)
            });
            pinger.expect_cache().returning(|| {
                Err(PingError::IcmpError)
            });
            // let pinger = IcmpDest::default();
            let res = block_on(pinger.ping());
            assert_eq!(PingError::IcmpError, res.err().unwrap());
            assert_eq!(PingError::IcmpError, pinger.cache().err().unwrap());
        }
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    log::error!("ERROR");
    log::warn!("WARN");
    log::info!("INFO");
    log::debug!("DEBUG");
    log::trace!("TRACE");

    // requires `cargo` feature, reading name, version, author, and description from `Cargo.toml`
    let matches = command!()
        .long_about("erich here")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to the configuration file")
                .value_parser(value_parser!(PathBuf))
                .default_value(DEFAULT_CONFIG_FILE.to_str().unwrap()),
        )
        .get_matches();
    log::info!(
        "config={:?}",
        matches.get_one::<String>("config")
    );
    
    let cfg = Config::builder().build().unwrap();
    log::info!("{:?}", cfg.try_deserialize::<HashMap<String, String>>().unwrap());

    let addr = "127.0.0.1:8080".parse().unwrap(); // replace with your server address
    let pinger = IcmpDest::new(addr); // replace with your server address
    // let is_up = is_server_up(addr).await;
    // let is_up = is_ping(addr).await;
    let is_up = pinger.ping().await;
    if is_up.unwrap() {
        println!("Server {addr} is up");
    } else {
        println!("Server {addr} is not up");
    }

}
