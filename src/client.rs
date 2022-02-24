use std::io;
use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;

use crate::common::{Message, MessageType};

pub struct Client {
    name: String,
    addr: SocketAddr,
}

impl Client {
    pub fn new(name: String, addr: SocketAddr) -> Client {
        Client {
            name,
            addr
        }
    }

    pub fn connect(self) {
        let mut stream = TcpStream::connect("127.0.0.1:80").expect("Couldn't connect to server...");

        let mut stream_cloned = stream.try_clone().unwrap();
        let name = self.name.clone();
        thread::spawn(move || {
            let mut buf = [0_u8; 1024];
            loop {
                buf.iter_mut().for_each(|element| *element = 0);
                let size = stream_cloned.read(&mut buf).unwrap();
                if size == 0 {
                    break;
                }
                let message: Message = bincode::deserialize(&buf).unwrap();
                if message.who != name {
                    println!("{}", message.to_string());
                }
            }
        });

        // Handshake
        let message = Message::handshake(self.name.clone());
        let bytes = bincode::serialize(&message).unwrap();
        stream.write(&bytes).unwrap();

        for line in io::stdin().lock().lines() {
            let message = Message::simple(self.name.clone(), line.unwrap());
            let bytes = bincode::serialize(&message).unwrap();
            stream.write(&bytes).unwrap();
        }
    }
}