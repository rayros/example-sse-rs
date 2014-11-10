extern crate time;
use std::io::Timer;
use std::time::Duration;
use std::collections::HashMap;
use std::io::{TcpListener, TcpStream};
mod server;

fn get_headers(status: &str, map: HashMap<&str, &str>) -> Vec<u8> {
  let mut result = format!("HTTP/1.1 {}\r\n", status);
  for (key, val) in map.iter() {
    result = result + format!("{}: {}\r\n", key, val);
  }
  result = result + "\r\n"; // end of headers
  result.into_bytes()
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3002");
    let s = server::Server::new(listener, handle_client);
    let run = s.run();
    match run {
        Ok(ok) => { println!("{}", ok) }
        Err(e) => { println!("Server failed: {}", e) }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut headers_map = HashMap::new();
    headers_map.insert("Content-Type", "text/event-stream"); 
    headers_map.insert("Connection", "keep-alive"); 
    headers_map.insert("Cache-Control", "no-cache");
    headers_map.insert("Access-Control-Allow-Origin", "*");
    let headers = get_headers("200 OK", headers_map);
    let mut buf = [0u8, ..1024];
    stream.read(buf);
    let respon: String = "data: elsds\n\n".to_string();
    let mut timer = Timer::new().unwrap();
    stream.write(headers.as_slice());
    loop { 
        stream.write(respon.clone().into_bytes().as_slice());
        //println!("{}", time::now().asctime());
        timer.sleep(Duration::milliseconds(100));
    }
    stream.close_write();
    drop(stream);
}
