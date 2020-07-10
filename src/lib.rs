use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use crate::errors::RedisCliError;

mod macros;
mod errors;

pub struct RedisCli {
    stream: TcpStream,
}

impl RedisCli {
    pub fn connect<T: ToSocketAddrs>(addr: T) -> Result<RedisCli, RedisCliError> {
        let stream = TcpStream::connect(addr)?;
        Ok(RedisCli {
            stream,
        })
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn test_connect_ok() {
        let srv = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = srv.local_addr().unwrap();
        assert_ok!(RedisCli::connect(&addr));
    }

    #[test]
    fn test_connect_err() {
        assert_err!(RedisCli::connect("127.0.0.1:0"));
    }

    #[test]
    fn test_send() {
        let srv = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = srv.local_addr().unwrap();

        let msg = "test";

        let mut redis = RedisCli::connect(&addr).unwrap();
        redis.send(msg.as_bytes());

        // handle server
        let mut buffer = [0; 4];

        let mut stream = srv.incoming().next().unwrap().unwrap();

        stream.read(&mut buffer).unwrap();

        assert_eq!(String::from_utf8_lossy(&buffer[..]), msg)
    }

    #[test]
    fn test_receive() {
        let srv = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = srv.local_addr().unwrap();

        let mut redis = RedisCli::connect(&addr).unwrap();

        let msg = "test_receive";

        let mut stream = srv.incoming().next().unwrap().unwrap();

        stream.write("test_receive".as_bytes()).unwrap();
        stream.flush().unwrap();

        assert_eq!(&redis.receive()[..12], msg)
    }
}