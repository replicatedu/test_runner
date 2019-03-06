

use std::io::Write;
        
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("gen_tests.rs");
    let mut f = std::fs::File::create(&destination).unwrap();

    write!(f,"
use test_runner::TestCommand;
use test_runner::*;

use std::process::{{self, Command, Output}};
").unwrap();
    let params = &["abc", "fooboo","asdasd","adsad"];
    for p in params {
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
        c.arg(\"echo hello\");
        c
    }};
    let mut cmd = TestCommand {{
        dir: \".\".to_string(),
        cmd: command,
    }};
    cmd.current_dir(\"{in_current_dir}\");
    let expected = \"{in_expected}\";
    let stdout = cmd.stdout();
    eqnice!(expected,stdout.trim());
}}
",
            in_name = p,
            in_command = "dir",
            in_current_dir = "../../",
            in_expected = "hello"
        ).unwrap();
    }
}