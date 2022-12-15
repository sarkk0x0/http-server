use std::{net::{TcpListener}, io::Read};
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap(); // unwrap returns a result or panic
        println!("Listening on address {}", self.addr);
        
        loop { // shortcut for while True
            
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                },
                                Err(e) => println!("Error received: {}", e)
                            }
                        },
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                },
                Err(e) => println!("Error establishing a connection: {}", e)
            }
        }
    }
}