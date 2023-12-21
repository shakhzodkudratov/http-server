use std::error::Error;
use std::net::{TcpListener, TcpStream};

use crate::http_server::http_server::HttpServer;

mod http_server;

const HOST: &str = "127.0.0.1";
const PORT: u32 = 8000;

fn main() {
    let hostname = format!("{}:{}", HOST.to_owned(), PORT);
    let listener = match TcpListener::bind(hostname.clone()) {
        Ok(listener) => listener,
        Err(error) => panic!("Error on creating server {}: {:?}", hostname, error),
    };

    println!("Server is running on {}", hostname);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => process_stream(stream),
            Err(error) => process_error(&error),
        }
    }
}

fn process_stream(stream: TcpStream) {
    let mut server = HttpServer::new(stream);
    let body = String::from("HTTP/1.1 200 OK\r\n\r\n<html><body>Hello world!</body></html>");
    server.response(&body);
}

fn process_error(error: &dyn Error) {
    println!("Error occured: {:?}", error);
}
