pub mod http_parser {
    use std::{collections::HashMap, io::Read, net::TcpStream};

    pub struct HttpParser {
        pub raw_stream: TcpStream,
        pub verb: String,
        pub protocol: String,
        lines: String,
        pub headers: HashMap<String, String>,
    }

    impl HttpParser {
        pub fn new(raw_stream: TcpStream) -> HttpParser {
            let mut parser: HttpParser = HttpParser {
                raw_stream,
                verb: String::new(),
                protocol: String::new(),
                lines: String::new(),
                headers: HashMap::new(),
            };

            parser.get_lines();
            parser.parse_verb_and_protocol();
            parser.parse_headers();

            parser
        }

        fn get_lines(&mut self) {
            match self.raw_stream.read_to_string(&mut self.lines) {
                Ok(_) => {}
                Err(error) => {
                    println!("Error reading string: {:?}", error);
                    return;
                }
            };
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
    }
}
