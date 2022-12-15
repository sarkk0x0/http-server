#![allow(dead_code)]
use std::{env, default};

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR");
    let public_path = env::var("PUBLIC_PATH").unwrap_or(format!("{}/public", default_path.to_string()));
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}