pub mod protocol;
pub mod storage;
pub mod server;
pub mod client;
pub mod extension;

pub use protocol::*;
pub use storage::*;
pub use server::Server;
pub use client::Client;
pub use extension::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub data_dir: String,
    pub log_dir: String,
    pub max_connections: usize,
    pub enable_ttl: bool,
    pub enable_pubsub: bool,
    pub enable_web: bool,
    pub web_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server_addr: "127.0.0.1:8080".to_string(),
            data_dir: "data".to_string(),
            log_dir: "logs".to_string(),
            max_connections: 100,
            enable_ttl: true,
            enable_pubsub: false,
            enable_web: false,
            web_port: 8081,
        }
    }
}