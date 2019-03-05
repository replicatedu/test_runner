#[macro_export]
macro_rules! cmdtest {
    ($name:ident, $cmd:expr, $fun:expr) => {
        #[test]
        fn $name() {
            let command = Command::new($cmd);
            let mut cmd = TestCommand {
                dir: "".to_string(),
                cmd: command,
            };
            $fun(cmd);
        }
    };
}

cmdtest!(test_ls, "ls", |mut cmd: TestCommand| {
    cmd.current_dir("..");
    let expected = "\
1:Watson
1:Sherlock
2:Holmes
3:Sherlock Holmes
5:Watson
";
    eqnice!(expected, cmd.stdout());
});

use std::io::Write;
        
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("gen_tests.rs");
    let mut f = std::fs::File::create(&destination).unwrap();

    write!(f,"
#[macro_use]
use test_runner::TestCommand;
#[macro_use]
use test_runner::*;

use std::process::{{self, Command}};
").unwrap();
    let params = &["abc", "fooboo","asdasd","adsad"];
    for p in params {
        write!(
            f,
            "
#[test]
fn {in_name}(){{
    let command = Command::new(\"{in_command}\");
    let mut cmd = TestCommand {{
        dir: \".\".to_string(),
        cmd: command,
    }};
    cmd.current_dir(\"{in_current_dir}\");
    let expected = \"{in_expected}\";
    eqnice!(expected,cmd.stdout());
}}
",
            in_name = p,
            in_command = "ls",
            in_current_dir = '.',
            in_expected = "asdsa"
        ).unwrap();
    }
}