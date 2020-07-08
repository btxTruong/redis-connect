use std::net::TcpStream;
use std::io::{Read, Write};
use ToString;
use std::borrow::Cow;

struct RedisCli {
    stream: TcpStream,
}

impl RedisCli {
    fn connect(host: &str, port: usize) -> RedisCli {
        RedisCli {
            stream: TcpStream::connect(format!("{}:{}", host, port.to_string())).unwrap()
        }
    }

    fn send(&mut self, data: &[u8]) {
        self.stream.write(data).unwrap();
        self.stream.flush().unwrap();
    }

    fn receive(&mut self) -> String {
        let mut buffer = [0; 512];
        self.stream.read(&mut buffer).unwrap();
        if let Cow::Owned(data) = String::from_utf8_lossy(&buffer[..]) {
            return data;
        }
        "abs".to_string()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading input");
    let mut input: Vec<&str> = input.split_whitespace().collect();

    let mut redis = RedisCli::connect("127.0.0.1", 6379);
    redis.send(input.join("").as_bytes());
    println!("{}", redis.receive())
}