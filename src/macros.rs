#[macro_export]
macro_rules! assert_ok {
    ($e:expr) => {
        use std::result::Result::*;
        match $e {
            Ok(v) => v,
            Err(e) => panic!("assertion failed, it's err"),
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($e:expr) => {
        use std::result::Result::*;
        match $e {
            Ok(v) => panic!("assertion failed, it's ok"),
            Err(e) => e,
        }
    };
}