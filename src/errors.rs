#[derive(Debug)]
pub enum RedisCliError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for RedisCliError {
    fn from(err: std::io::Error) -> Self {
        RedisCliError::IOError(err)
    }
}