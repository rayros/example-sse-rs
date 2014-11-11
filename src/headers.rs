use std::io::TcpStream;
use std::collections::HashMap;

pub struct HeadersInfo<'a> {
    pub vec: Vec<u8>,
    pub status: &'a str,
    pub hash_map: HashMap<&'a str, &'a str>,
}
pub struct Headers<'a> { 
    pub info: HeadersInfo<'a> 
}
impl Headers<'static> {
    pub fn new<'a>() -> Headers<'a> {
        let status = "200 OK";
        let mut hash_map = HashMap::new();
        hash_map.insert("Content-Type", "text/event-stream"); 
        hash_map.insert("Connection", "keep-alive"); 
        hash_map.insert("Cache-Control", "no-cache");
        hash_map.insert("Access-Control-Allow-Origin", "*");
        let headers = Headers::get_headers(status, hash_map.clone());
        Headers { info: HeadersInfo { 
            status: status, 
            vec: headers,
            hash_map: hash_map,
        } }
    }
    pub fn to_stream(self, mut stream: TcpStream) -> Result<&'static str, &'static str> {
        stream.write(self.info.vec.as_slice()).unwrap();
        drop(stream);
        Ok("Wporzo")
    }
    fn get_headers(status: &str, map: HashMap<&str, &str>) -> Vec<u8> {
        let mut result = format!("HTTP/1.1 {}\r\n", status);
        for (key, val) in map.iter() {
            result = result + format!("{}: {}\r\n", key, val);
        }
        result = result + "\r\n"; // end of headers
        result.into_bytes()
    }
}
