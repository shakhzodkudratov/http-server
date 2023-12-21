use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

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
            Ok(stream) => process_stream(&stream),
            Err(error) => process_error(&error),
        }
    }
}

fn process_stream(mut stream: &TcpStream) {
    let mut lines = String::new();
    match stream.read_to_string(&mut lines) {
        Ok(_) => {}
        Err(error) => println!("Error reading string: {:?}", error),
    }

    lines.split("\n").for_each(|line| {
        println!("Line {:?}", line);
    });
    // println!("Incoming stream: {:?}", lines);
}

fn process_error(error: &dyn Error) {
    println!("Error occured: {:?}", error);
}
