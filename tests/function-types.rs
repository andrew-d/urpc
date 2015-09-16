#![feature(trace_macros, type_macros)]

extern crate libc;
extern crate rustc_serialize as rustc_serialize;
extern crate unix_socket;

#[macro_use]
extern crate urpc;

use unix_socket::UnixStream;

urpc! {
    pub interface testif {
        fn add_one(x: u32) -> u32 {}
        fn no_args() -> u32 {}

        fn implicit_return(x: u32) {}
    }
}

struct Impl;

impl testif::Methods for Impl {
    fn add_one(&mut self, x: u32) -> urpc::Result<u32> {
        Ok(x + 1)
    }

    fn no_args(&mut self) -> urpc::Result<u32> {
        Ok(1234)
    }

    fn implicit_return(&mut self, x: u32) {
        println!("called with x = {}", x);
    }
}

#[test]
fn test_simple_methods() {
    use testif::Methods;

    let (s1, s2) = UnixStream::unnamed().unwrap();

    let pid = unsafe { libc::fork() };
    assert!(pid >= 0);

    match pid {
        0 => {
            drop(s2);

            testif::serve(Impl, s1).unwrap();
        }
        _ => {
            drop(s1);
            let mut client = testif::Client::new(s2);

            match client.add_one(42) {
                Ok(num) => assert_eq!(num, 43),
                Err(e) => panic!("expected no error: {:?}", e),
            };

            match client.no_args() {
                Ok(num) => assert_eq!(num, 1234),
                Err(e) => panic!("expected no error: {:?}", e),
            };

            match client.implicit_return() {
                Ok(()) => {},
                Err(e) => panic!("expected no error: {:?}", e),
            };
        }
    }
}
