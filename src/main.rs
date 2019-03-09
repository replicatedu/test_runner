extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::process::{Command};
use std::fs;
use test_runner::{TestCommand};

//returns a command setup ready to run the tests
fn setup_command(test_command:&str,test_directory:String)->TestCommand{
    let command = if cfg!(target_os = "windows") {{
        let mut c = Command::new("cmd");
        c.args(&["/C", test_command]);
        c
    }} else {{
        let mut c = Command::new("sh");
        c.arg("-c");
        c.arg("test_command");
        c
    }};
    let mut cmd = TestCommand {
        dir: ".".to_string(),
        cmd: command,
    };
    cmd.current_dir(test_directory);
    cmd
}

//tests to ensure an error propigates
fn test_assert_err(mut tester:TestCommand){
    tester.assert_err();
}    

//tests to see if there is an exit code
fn test_assert_exit_code(mut tester:TestCommand, TestConfig:Test){
    tester.assert_exit_code(TestConfig.exit_code.expect("test did not have a proper exit code"));
}    

//test to see if there is a stderr
fn test_assert_non_empty_stderr(mut tester:TestCommand){
    tester.assert_non_empty_stderr();
}    

//checks to see if output threshold is made
fn test_expect_output_threshold(mut tester:TestCommand,TestConfig:Test){
    tester.expect_output_threshold( TestConfig.target_threshold.expect("test did not have a threshold"),
                                    &TestConfig.expected.expect("test did not have a expected var"));
}    

//checks to see if output identical
fn test_output_is_expected(mut tester:TestCommand,TestConfig:Test){
    tester.expect_output_threshold(1.0,&TestConfig.expected.expect("test did not have a expected var"));
}    

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct TestDoc {
    test_file_description: Option<String>,
    test: Option<Vec<Test>>,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize)]
struct Test {
    description: Option<String>,
    test_type: Option<String>,
    test_directory: Option<String>,
    exit_code: Option<i32>,
    target_threshold: Option<f64>,
    cmd: Option<String>,
    expected: Option<String>,
    points: Option<u64>,
}


fn broker_tests(test:&Test){
    let s = test.test_type.unwrap().clone();
    let s_slice: &str = &s[..];
    match  s_slice{
        "test_assert_err" => println!("test "),
        _ => println!("olther")
    }
}

fn main(){
    let filename = "example/test.toml";
    let contents = match fs::read_to_string(&filename) {
            Ok(contents) => contents,
            Err(_) => panic!("should not have happened"),
    };

    
    let decoded: TestDoc = toml::from_str(&contents).unwrap();
    //println!("{:#?}", decoded);
    let tests = decoded.test.unwrap();
    for test in tests.iter(){
        println!("{:?}",test);
        //broker_tests(&test);
    }
}