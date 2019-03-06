
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

        
fn main() {
    //get the environmental variable for the test directory
    // let folder = env::var("REPLICATED_EDU_TEST_DIR")
    //     .expect("set the env variable for the REPLICATED_EDU_TEST_DIR to the test directory");
    // println!("{:?}", folder);
    
    //set the destination directory
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("gen_tests.rs");
    let mut f = std::fs::File::create(&destination).unwrap();
    
    let test_std_out = "
    let stdout = cmd.stdout();
    eqnice!(expected,stdout.trim());
    ".to_string();
    
    write_test_header(&f);
    write_test_to_file(&f,"test_dir","dir","../","hello2",&test_std_out);
}