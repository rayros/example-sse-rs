extern crate time;
extern crate regex;
use std::io::Timer;
use std::time::Duration;
use std::io::{TcpListener, TcpStream};
mod server;
mod request;
mod headers;

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

    let request = request::Request::new(stream.clone());
    let headers = headers::Headers::new();
    let respon: String = "data: elsds\n\n".to_string();
    let mut timer = Timer::new().unwrap();
    println!("{}", request.path);
    headers.to_stream(stream.clone());
    loop { 
        stream.write(respon.clone().into_bytes().as_slice());
        timer.sleep(Duration::milliseconds(100));
    }
    stream.close_write();
    drop(stream);
}
