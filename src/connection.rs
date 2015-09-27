//! NSQ Connection

use std::net::TcpStream;

pub struct Connection {
    tcpStream: TcpStream
}

impl Connection {
    pub fn new(host: &str) -> Connection {
        Connection{
            tcpStream: TcpStream::connect(host).unwrap(),
        }
    }
}
