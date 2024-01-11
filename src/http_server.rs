pub mod http_server {
    use std::{io::Write, net::TcpStream};

    use crate::logic::request::HttpRequest;

    pub struct HttpServer {
        pub request: HttpRequest,
        stream: TcpStream,
    }

    impl HttpServer {
        pub fn new(stream: TcpStream) -> HttpServer {
            HttpServer {
                request: HttpRequest::new(),
                stream,
            }
        }

        pub fn parse(&mut self) {
            self.request.parse(&mut self.stream);
        }

        pub fn response(&mut self, body: &str) {
            let bytes = body.as_bytes();
            match self.stream.write_all(bytes) {
                Ok(_) => (),
                Err(error) => {
                    println!("Error occurred while writing into stream: {:?}", error);
                }
            }
        }
    }
}
