//use std::process::Command;
use test_runner::{TestDoc, run_test_file};

fn main() {
    let filename = "example/test.toml";
    let scores = run_test_file(filename.to_string());
    dbg!(scores);
}
