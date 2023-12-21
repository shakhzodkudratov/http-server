use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::http_parser::http_parser::HttpParser;

mod http_parser;

const HOST: &str = "127.0.0.1";
const PORT: u32 = 8000;

fn main() {
    println!("Here");
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
    let parser = HttpParser::new(stream);
}

fn process_error(error: &dyn Error) {
    println!("Error occured: {:?}", error);
}
