pub mod http_server {
    use std::{
        collections::HashMap,
        io::{BufRead, BufReader, Write},
        net::TcpStream,
    };

    pub struct HttpServer {
        pub raw_stream: TcpStream,
        pub verb: String,
        pub protocol: String,
        lines: String,
        pub headers: HashMap<String, String>,
    }

    impl HttpServer {
        pub fn new(raw_stream: TcpStream) -> HttpServer {
            let mut server = HttpServer {
                raw_stream,
                verb: String::new(),
                protocol: String::new(),
                lines: String::new(),
                headers: HashMap::new(),
            };

            server.get_lines();
            server.parse_verb_and_protocol();
            server.parse_headers();

            server
        }

        fn get_lines(&mut self) {
            let reader = BufReader::new(&mut self.raw_stream);
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
                Some(string) => self.verb.push_str(string),
                None => {
                    println!("Failed to parse HTTP verb");
                    return;
                }
            }

            match iter.next() {
                Some(string) => self.protocol.push_str(string),
                None => {
                    println!("Failed to parse HTTP protocol");
                    return;
                }
            }

            println!("HTTP verb: {}\nHTTP protocol: {}", self.verb, self.protocol);
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

        pub fn response(&mut self, body: &str) {
            let bytes = body.as_bytes();
            match self.raw_stream.write_all(bytes) {
                Ok(_) => (),
                Err(error) => {
                    println!("Error occurred while writing into stream: {:?}", error);
                }
            }
        }
    }
}
