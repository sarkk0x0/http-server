use std::{net::{TcpListener, TcpStream}, io::Read};
use super::http::{Request, Response, StatusCode};
use super::http::request::ParseError;
use std::io::Write;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("unable to parse request: {}", e);
        Response::new(StatusCode::NotFound, None)
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap(); // unwrap returns a result or panic
        println!("Listening on address {}", self.addr);
        
        loop { // shortcut for while True
            
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    handler.handle_request(&req)
                                },
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
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