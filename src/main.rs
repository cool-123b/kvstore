mod protocol;
mod storage;
mod server;
mod client;
mod extension;

use std::env;
use kvstore::{Config, Server, Client};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "server" => {
            let addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
            run_server(addr);
        }
        "client" => {
            let addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
            run_client(addr);
        }
        _ => {
            println!("❌ 未知模式: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("用法:");
    println!("  cargo run -- server [地址]    启动服务器");
    println!("  cargo run -- client [地址]    启动客户端");
}

fn run_server(addr: &str) {
    let config = Config::default();
    let server = Server::new(&config);
    if let Err(e) = server.run(addr) {
        eprintln!("❌ 服务器错误: {}", e);
    }
}

fn run_client(addr: &str) {
    let mut client = Client::new(addr);
    if let Err(e) = client.run() {
        eprintln!("❌ 客户端错误: {}", e);
    }
}