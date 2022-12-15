use std::fs;

use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use super::http::request::ParseError;

pub struct WebsiteHandler{
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    pub fn read_file(&self, path: &str) -> Option<String> {
        let file_path = format!("{}/{}", self.public_path, path);
        // let read_string = fs::read_to_string(file_path);
        // dbg!(read_string);
        fs::read_to_string(file_path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                    _ => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}