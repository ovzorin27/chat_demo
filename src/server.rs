use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use bincode::Result;
use crate::common::Message;

pub struct Server {
    addr: SocketAddr,
    // clients
}

impl Server {
    pub fn new(addr: SocketAddr) -> Server {
        Server {
            addr
        }
    }

    pub fn listen(mut self) {
        let (tx, rx) = channel();

        let clients = Arc::new(Mutex::new(Vec::new()));

        let clients_cloned = Arc::clone(&clients);
        let handle = thread::spawn(move || {
            let listener = TcpListener::bind(self.addr).unwrap();
            // accept connections and process them serially
            for stream in listener.incoming() {
                let tx_cloned = tx.clone();
                let stream = stream.unwrap();
                let stream_cloned = stream.try_clone().unwrap();
                let mut vec_streams = clients_cloned.lock().unwrap();
                vec_streams.push(stream_cloned);
                self.handle_client(stream, tx_cloned);
            }
        });

        let clients_cloned = Arc::clone(&clients);
        // This thread receive messages from handle clients thread
        // and send it to all clients
        let handle2 = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("received: {}", msg);
                let bytes = bincode::serialize(&msg).unwrap();
                let mut l = clients_cloned.lock().unwrap();
                for v in l.iter_mut() {
                    v.write(&bytes);
                }
            }
        });

        handle.join();
        handle2.join();
    }

    fn handle_client(&mut self, mut stream: TcpStream, tx: Sender<Message>) {
        // Thread for handle clients
        thread::spawn(move || {
            let mut buf = [0_u8; 1024];
            loop {
                buf.iter_mut().for_each(|element| *element = 0);
                let read_result = stream.read(&mut buf);
                match read_result {
                    Ok(size) if size > 0 => {
                        let message: Message = bincode::deserialize(&buf).unwrap();
                        println!("{}", message);
                        tx.send(message);
                    }
                    Ok(size) => {
                        // todo
                        break;
                    },
                    Err(_) => {
                        // todo
                        break;
                    }
                }
            }
        });
    }
}