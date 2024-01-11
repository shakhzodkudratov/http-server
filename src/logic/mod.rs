use std::fmt;

pub mod request;

pub enum HttpProtocol {
    V1_1,
}

impl HttpProtocol {
    pub fn parse(input: &str) -> Option<HttpProtocol> {
        None
    }
}

impl fmt::Display for HttpProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::V1_1 => "v1.1",
        };

        write!(f, "{}", value)
    }
}

pub enum HttpVerb {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl HttpVerb {
    pub fn parse(input: &str) -> Option<HttpVerb> {
        None
    }
}

impl fmt::Display for HttpVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
            Self::DELETE => "DELETE",
            Self::OPTIONS => "OPTIONS",
            Self::HEAD => "HEAD",
        };

        write!(f, "{}", value)
    }
}
