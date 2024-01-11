use crate::logic::HttpProtocol;
use crate::logic::HttpVerb;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

pub struct HttpRequest {
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub query: HashMap<String, String>,
    pub verb: Option<HttpVerb>,
    pub protocol: Option<HttpProtocol>,

    lines: String,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        HttpRequest {
            headers: HashMap::new(),
            body: Some(String::new()),
            query: HashMap::new(),
            verb: None,
            protocol: None,

            lines: String::new(),
        }
    }

    pub fn parse(&mut self, stream: &mut TcpStream) {
        self.get_lines(stream);
        self.parse_verb_and_protocol();
        self.parse_headers();
    }

    fn get_lines(&mut self, stream: &mut TcpStream) {
        let reader = BufReader::new(stream);
        let request: Vec<_> = reader
            .lines()
            .map(|result| match result {
                Ok(string) => string,
                Err(error) => {
                    println!("Error unpacking string from stream: {:?}", error);
                    String::new()
                }
            })
            .take_while(|line| !line.is_empty())
            .collect();
        self.lines = request.join("\r\n");
    }

    fn parse_verb_and_protocol(&mut self) {
        let mut verb_and_protocol = String::new();

        match self.lines.split_once("\r\n") {
            Some(line) => verb_and_protocol.push_str(line.0),
            None => {
                println!("Failed to parse first line");
                return;
            }
        };

        let mut iter = verb_and_protocol.splitn(2, " / ");
        match iter.next() {
            Some(string) => self.verb = HttpVerb::parse(string),
            None => {
                println!("Failed to parse HTTP verb");
                return;
            }
        }

        match iter.next() {
            Some(string) => self.protocol = HttpProtocol::parse(string),
            None => {
                println!("Failed to parse HTTP protocol");
                return;
            }
        }

        match &self.protocol {
            Some(protocol) => println!("HTTP protocol: {}", protocol),
            None => println!("HTTP protocol is unknown"),
        }

        match &self.verb {
            Some(verb) => println!("HTTP verb: {}", verb),
            None => println!("HTTP verb is unknown"),
        }
    }

    fn parse_headers(&mut self) {
        self.lines
            .split("\r\n")
            .skip(1)
            .take_while(|line| line.len() != 0)
            .for_each(|line| match line.split_once(":") {
                Some((key, value)) => {
                    self.headers.insert(key.to_owned(), value.to_owned());
                }
                None => {
                    println!("Failed to parse header: {}", line)
                }
            });

        println!("Headers: {:?}", self.headers);
    }
}
