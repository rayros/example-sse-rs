use std::str::from_utf8;
use std::io::TcpStream;
use regex::Regex;

pub struct Request {
    pub path: String,
    buffor: [u8, ..1024],
    buffor_text: String,
}
impl Request {
    pub fn new(mut stream: TcpStream) -> Request {
        let mut buf = [0u8, ..1024];
        stream.read(buf);
        stream.close_read();
        let buf_text = from_utf8(buf).unwrap();
        let re = match Regex::new(r"GET ([^ ]+) .+") {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
        };
        let mut path = "";
        match re.captures(buf_text) {
            Some(text) => {
                path = text.at(1);
            },
            None => println!("Request: Lack of path."),
        };
        Request { path: path.to_string(), buffor: buf, buffor_text: buf_text.to_string() }
    }
}
