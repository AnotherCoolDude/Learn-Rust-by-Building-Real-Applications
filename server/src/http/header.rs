use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Header {
    data: HashMap<String, String>,
}

impl<'buf> Header {
    pub fn new() -> Self {
        Self {data: HashMap::new()}
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

}

impl<'buf> From<&'buf str> for Header {
    fn from(s: &'buf str) -> Self {
        let mut header = Header::new();

        for line in s.split("\n") {
            let mut key = "";
            let mut val = "";

            if let Some(i) = line.find(':') {
                key = &line[..i].trim();
                val = &line[i + 2..].trim();
            }

            if key == "" {
                continue;
            }
            header.data.insert(key.to_string(), val.to_string());
        }

        header
    }
}

impl<'buf> Display for Header {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut header_str = String::from("");
        for (key, val) in self.data.iter() {
            header_str = format!("{}{}: {}\r\n", header_str, key, val);
        }
        write!(f, "{}", header_str)
    }
}