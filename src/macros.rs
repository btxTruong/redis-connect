#[macro_export]
macro_rules! assert_ok {
    ($e:expr) => {
        use std::result::Result::*;
        match $e {
            Ok(v) => v,
            Err(_e) => panic!("assertion failed, it's err"),
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($e:expr) => {
        use std::result::Result::*;
        match $e {
            Ok(_v) => panic!("assertion failed, it's ok"),
            Err(e) => e,
        }
    };
}