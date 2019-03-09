
use std::io::Write;
use std::env;
use std::fs::File;

fn write_test_header(mut f:&File){
    write!(f,"
use test_runner::TestCommand;
use test_runner::*;
use std::process::{{Command}};
    ").unwrap();
}

pub fn write_test_to_file( mut f:&File,
                           name:&str, 
                           command:&str, 
                           current_dir:&str, 
                           expected_output:&str,
                           rust_macro_test_string:&str ){
    write!(
        f,
        "

#[test]
fn {in_name}(){{
    let command = if cfg!(target_os = \"windows\") {{
        let mut c = Command::new(\"cmd\");
        c.args(&[\"/C\", \"{in_command}\"]);
        c
    }} else {{
        let mut c = Command::new(\"sh\");
        c.arg(\"-c\");
        c.arg(\"{in_command}\");
        c
    }};
    let mut cmd = TestCommand {{
        dir: \".\".to_string(),
        cmd: command,
    }};
    cmd.current_dir(\"{in_current_dir}\");
    let expected = \"{in_expected}\";
    {in_rust_macro_test_string}
}}
",
    in_name = name,
    in_command = command,
    in_current_dir = current_dir,
    in_expected = expected_output,
    in_rust_macro_test_string = rust_macro_test_string
    ).unwrap();
}

//generates a test stub for the test string
fn expect_stdout_test_stub()->String{
    "
    let stdout = cmd.stdout();
    cmd.expect_output_threshold(1.00,expected.trim());
    ".to_string()
} 

//generates a test stub for the threshold test
fn stdout_threshold_test_stub(threshold:f64)->String{
    format!("
    cmd.expect_output_threshold({},expected.trim());
    ",threshold).to_string()
} 

//generates a test stub for an error test
fn non_empty_stderr_test_stub()->String{
    "
    cmd.assert_non_empty_stderr();
    ".to_string()
} 

//generates a test stub to get a certain exit code
fn assert_exit_code_test_stub(exit_code:i64)->String{
    "
    cmd.assert_exit_code();
    ".to_string()
}


fn main() {
    //get the environmental variable for the test directory
    //in powershell: $env:REPLICATED_EDU_TEST_FILE = "./example/test.toml"
    let folder = env::var("REPLICATED_EDU_TEST_FILE")
        .expect("set the env variable for the REPLICATED_EDU_TEST_DIR to the test directory: 
                 Powershell: $env:REPLICATED_EDU_TEST_FILE = \"./example/test.toml\"
                 Linux: set REPLICATED_EDU_TEST_FILE ./example/test.toml\n\n");
    
    //set the destination directory for the test executable
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("gen_tests.rs");
    let mut f = std::fs::File::create(&destination).unwrap();
    

    //read the test file and generate the tests
    


    let test_std_out = expect_stdout_test_stub();
    write_test_header(&f);
    
    write_test_to_file(&f,"test_dir","echo hello work","../","fsadfasdfasdwork",&test_std_out);
    
    let test_threshold = stdout_threshold_test_stub(0.90);
    write_test_to_file(&f,"test2_dir","echo hello work","../","asdfasfdsdasdfasd work",&test_threshold);
}