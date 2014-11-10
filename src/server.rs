extern crate std;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

type ResultTcp = Result<std::io::net::tcp::TcpListener, std::io::IoError>;

pub struct Server {
    listener: ResultTcp,
    client_handler: fn(mut stream: TcpStream),
}
impl Server {
    pub fn new(listener: ResultTcp, client_handler: fn(mut stream: TcpStream)) -> Server {
        Server { listener: listener, client_handler: client_handler }
    }
    pub fn run(self) -> Result<&'static str, &'static str> {
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
