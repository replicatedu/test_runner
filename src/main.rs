use std::env;
use test_runner::{TestDoc, run_test_file};
use walkdir::{DirEntry, WalkDir};
use std::str;

use std::fs::OpenOptions;
use std::fs;
use std::io::Write;


pub fn write_file(filepath: &str, contents: &str) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath)
    {
        Ok(ref mut file) => {
            file.set_len(0);
            writeln!(file, "{}", contents).unwrap();
        }
        Err(err) => {
            panic!("Failed to open log file: {}", err);
        }
    }
}


pub fn should_ignore(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".git"))
        .unwrap_or(false)
}

pub fn find_test_files(test_file_filter: &str,dir: &str) -> Vec<String> {
    let mut tests = Vec::new();
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !should_ignore(e)) {
       let entry = entry.unwrap().path().display().to_string();

       if entry.contains(test_file_filter) {
            let s = entry.to_string();
            tests.push(s)
        }
    }
    tests
}
pub fn run_tests(test_files:Vec<String>) ->Vec<Vec<u64>>{
    let mut ret_scores = Vec::new();
    for file in test_files{
        let scores = run_test_file(file);
        ret_scores.push(scores);
    }
    ret_scores
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let test_filenames = &args[1];
    let outfile_scores = &args[2];
    let test_files = find_test_files(test_filenames,".");
    dbg!(&test_files);
    let scores = run_tests(test_files);
    dbg!(&scores);

    write_file(outfile_scores, &format!("{:?}",scores));
}
