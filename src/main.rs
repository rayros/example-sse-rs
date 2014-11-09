extern crate time;

use std::io::Timer;
use std::time::Duration;
use std::str::from_utf8;
use std::collections::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn get_headers(status: &str, map: HashMap<&str, &str>) -> Vec<u8> {
  let mut result = format!("HTTP/1.1 {}\r\n", status);
  for (key, val) in map.iter() {
    result = result + format!("{}: {}\r\n", key, val);
  }
  result = result + "\r\n"; // end of headers
  result.into_bytes()
}

type ResultTcp = Result<std::io::net::tcp::TcpListener, std::io::IoError>;

struct Server {
    listener: ResultTcp,
    client_handler: fn(mut stream: TcpStream),
}
impl Server {
    fn new(listener: ResultTcp, client_handler: fn(mut stream: TcpStream)) -> Server {
        Server { listener: listener, client_handler: client_handler }
    }
    fn run(self) -> Result<&'static str, &'static str> {
        let mut acceptor = self.listener.listen();
        for stream in acceptor.incoming() {
            match stream {
                Err(e) => { println!("Connection failed: {}", e) }
                Ok(stream) => {
                    // connection succeeded
                    let client_handler_clone = self.client_handler.clone();
                    spawn(proc() {
                       (client_handler_clone)(stream)
                    })
                }
            }
        }
        // close the socket server
        drop(acceptor);
        return Ok("Server close success.")
    }
}
fn main() {

    let listener = TcpListener::bind("127.0.0.1:3002");
    let s = Server::new(listener, handle_client);
    let run = s.run();
    match run {
        Ok(ok) => { println!("{}", ok) }
        Err(e) => { println!("Server failed: {}", e) }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut headers_map = HashMap::new();
    headers_map.insert("Content-Type", "text/event-stream, charset=utf8"); 
    headers_map.insert("Connection", "keep-alive"); 
    headers_map.insert("Cache-Control", "no-cache");
    headers_map.insert("Transfer-Encoding", "chunked");
    headers_map.insert("Access-Control-Allow-Origin", "*");

    let headers = get_headers("200 OK", headers_map);
    let response = headers;
    let mut buf = [0u8, ..1024];
    stream.read(buf);
    let req = from_utf8(buf).expect("Buffer fail");
    let response_string = from_utf8(response.as_slice()).expect("Response fail");
    println!("\nREQUEST:\n{:s}", req);
    println!("\nRESPONSE:\n{:s}", response_string);
    stream.write(response.as_slice());
    let respons: &[u8] = b"11\r\ndata: testo\r\n0\r\n\r\n";
    let mut timer = Timer::new().unwrap();
    loop { 
        stream.write(respons); 
        println!("{}", time::now().asctime());
        timer.sleep(Duration::milliseconds(100));
    }
    stream.close_write();
    drop(stream);
}
