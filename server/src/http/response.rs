use std::io::{Result as IoResult, Write};
use super::Header;

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    header: Header,
}

impl<'buf> Response {
    pub fn new(status_code: StatusCode, body: Option<String>, header: Option<Header>) -> Self {
        match header {
            Some(h) => Response { status_code, body, header: h },
            None => Response { status_code, body, header: Header::new() }
        }
        
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        println!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.header.to_string().as_str(),
            body
        );

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.header.to_string().as_str(),
            body
        )
    }
}
