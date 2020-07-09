use std::net::TcpStream;
use std::io::{Read, Write};

pub struct RedisCli {
    stream: TcpStream,
}

impl RedisCli {
    pub fn connect(host: &str, port: usize) -> RedisCli {
        RedisCli {
            stream: TcpStream::connect(format!("{}:{}", host, port.to_string())).unwrap()
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.stream.write(data).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn receive(&mut self) -> String {
        let mut buffer = [0; 512];
        self.stream.read(&mut buffer).unwrap();
        String::from_utf8_lossy(&buffer[..]).into_owned()
    }
}