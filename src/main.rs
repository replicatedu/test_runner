extern crate toml;
#[macro_use]
extern crate serde_derive;
use std::panic;
use std::fs;
//use std::process::Command;
use test_runner::{TestDoc, broker_test};


fn main() {
    let filename = "example/test.toml";
    let contents = match fs::read_to_string(&filename) {
        Ok(contents) => contents,
        Err(_) => panic!("should not have happened"),
    };

    let decoded: TestDoc = toml::from_str(&contents).unwrap();
    //println!("{:#?}", decoded);
    let tests = decoded.test.unwrap();
    for test in tests.iter() {
        println!("{:?}", test);
        let result = panic::catch_unwind(|| {
            broker_test(test);
        });
    }
}
