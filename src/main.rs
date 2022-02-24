mod server;
mod client;
mod common;

use std::net::SocketAddr;
use std::str::FromStr;
use server::Server;
use client::Client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    if args[1].eq("server") {
        let server = Server::new(SocketAddr::from_str("127.0.0.1:80").unwrap());
        server.listen();
    } else if args[1].eq("client") {
        if args.len() >= 3 {
            let name = args[2].clone();
            let client = Client::new(name ,SocketAddr::from_str("127.0.0.1:80").unwrap());
            client.connect();
        } else {
            print_help();
            return;
        }
    } else {
        print_help();
        return;
    }
}

fn print_help() {
    println!("Usage:\nFor server: `chat server`\nFor client: `chat client <name>`");
}
