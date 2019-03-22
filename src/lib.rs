
use std::fs;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{self, Command};
use std::panic;

extern crate toml;

#[macro_use]
extern crate serde_derive;

extern crate difference;
use difference::{Changeset, Difference};
extern crate term;

//gets a percentage difference of the test
pub fn percentage_diff(got: &str, expected: &str) -> (f64, String) {
    let mut t = term::stdout().unwrap();
    let Changeset { diffs, .. } = Changeset::new(got.trim(), expected.trim(), "");
    let mut len_same: usize = 0;
    let mut len_rem: usize = 0;
    let mut outdiff: String = String::new();
    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                len_same += x.len();
                outdiff.push_str(&format!("={}", x));
            }
            Difference::Add(ref x) => {
                len_rem += x.len();
                outdiff.push_str(&format!("+{}", x));
            }
            Difference::Rem(ref x) => {
                len_rem += x.len();
                outdiff.push_str(&format!("-{}", x));
            }
        }
    }
    t.fg(term::color::CYAN).unwrap();
    let total = len_same + len_rem;
    let percentage: f64 = len_same as f64 / total as f64;

    //writeln!(t, "%{} ({}/{})", percentage*100.0,len_same,total).expect("failed to print line");
    t.reset().unwrap();
    t.flush().unwrap();

    (percentage, outdiff)
}

/// A simple wrapper around a process::Command with some verification conveniences.
/// this was heavily influenced by the excellent rust writer: BurntSushi
/// https://github.com/BurntSushi/ripgrep/blob/7a6a40bae18f89bcfc6997479d7202d2c098e964/tests/util.rs
#[derive(Debug)]
pub struct TestCommand {
    /// The dir used to launched this command.
    pub dir: String,
    /// The actual command we use to control the process.
    pub cmd: Command,
}

impl TestCommand {
    /// Returns a mutable reference to the underlying command.
    pub fn cmd(&mut self) -> &mut Command {
        &mut self.cmd
    }

    /// Add an argument to pass to the command.
    pub fn arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut TestCommand {
        self.cmd.arg(arg);
        self
    }

    /// Add any number of arguments to the command.
    pub fn args<I, A>(&mut self, args: I) -> &mut TestCommand
    where
        I: IntoIterator<Item = A>,
        A: AsRef<OsStr>,
    {
        self.cmd.args(args);
        self
    }

    /// Set the working directory for this command.
    ///
    /// Note that this does not need to be called normally, since the creation
    /// of this TestCommand causes its working directory to be set to the
    /// test's directory automatically.
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut TestCommand {
        self.cmd.current_dir(dir);
        self
    }

    /// Runs and captures the stdout of the given command.
    pub fn stdout(&mut self) -> String {
        let o = self.output();
        let stdout = String::from_utf8_lossy(&o.stdout);
        match stdout.parse() {
            Ok(t) => t,
            Err(err) => {
                panic!("could not convert from string: {:?}\n\n{}", err, stdout);
            }
        }
    }

    /// Gets the output of a command. If the command failed, then this panics.
    pub fn output(&mut self) -> process::Output {
        let output = self.cmd.output().unwrap();
        self.expect_success(output)
    }

    /// Runs the command and asserts that it resulted in an error exit code.
    pub fn assert_err(&mut self) {
        let o = self.cmd.output().unwrap();
        if o.status.success() {
            panic!(
                "\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
                 \n===== {:?} =====\n\
                 command succeeded but expected failure!\
                 \ncwd: {}\
                 \nstatus: {}\
                 \nstdout: {}\n\nstderr: {}\
                 \n=====\n",
                self.cmd,
                self.dir,
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }

    /// Runs the command and asserts that its exit code matches expected exit
    /// code.
    pub fn assert_exit_code(&mut self, expected_code: i32) {
        let o = self.cmd.output().unwrap();
        let code = o.status.code().unwrap();
        assert_eq!(
            expected_code,
            code,
            "\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
             \n===== {:?} =====\n\
             expected exit code did not match\
             \nexpected: {}\
             \nfound: {}\
             \nstdout: {}\n\nstderr: {}\
             \n=====\n",
            self.cmd,
            expected_code,
            code,
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
    }

    /// Runs the command and asserts that something was printed to stderr.
    pub fn assert_non_empty_stderr(&mut self) {
        let o = self.cmd.output().unwrap();
        if o.status.success() || o.stderr.is_empty() {
            panic!(
                "\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
                 \n===== {:?} =====\n\
                 command succeeded but expected failure!\
                 \ncwd: {}\
                 \nstatus: {}\
                 \nstdout: {}\n\nstderr: {}\
                 \n=====\n",
                self.cmd,
                self.dir,
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }

    pub fn expect_output_threshold(&mut self, target_threshold: f64, expected: &str) {
        let o = self.cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&o.stdout);
        println!("{}", expected);
        let (perc_diff, out) = percentage_diff(&stdout, &expected);
        if target_threshold > perc_diff {
            panic!(
                "\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
                 \n===== {:?} =====\n\
                 only {} matched expected output, required {}!\
                 \ncwd: {}\
                 \nstatus: {}\
                 \nstdout: {}\nstderr: {}\
                 \ndiff:\n{}\n\
                 \n=====\n",
                self.cmd,
                perc_diff,
                target_threshold,
                self.dir,
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr),
                out
            );
        }
    }

    fn expect_success(&self, o: process::Output) -> process::Output {
        if !o.status.success() {
            let suggest = if o.stderr.is_empty() {
                "\n\nDid your search end up with no results?".to_string()
            } else {
                "".to_string()
            };

            panic!(
                "\n==========\n\
                 command failed but expected success!\
                 {}\
                 \ncommand: {:?}\
                 \ncwd: {}\
                 \nstatus: {}\
                 \nstdout: {}\
                 \nstderr: {}\
                 \n==========\n",
                suggest,
                self.cmd,
                self.dir,
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
        o
    }
}


//returns a command setup ready to run the tests
fn setup_command(test_command: &str, test_directory: String) -> TestCommand {
    let command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    let mut cmd = TestCommand {
        dir: ".".to_string(),
        cmd: command,
    };
    cmd.current_dir(test_directory);
    cmd
}

//tests to ensure an error propigates
fn test_assert_err(mut tester: TestCommand) {
    tester.assert_err();
}

//tests to see if there is an exit code
fn test_assert_exit_code(mut tester: TestCommand, test_config: &Test) {
    tester.assert_exit_code(
        test_config
            .exit_code()
            .expect("test did not have a proper exit code"),
    );
}

//test to see if there is a stderr
fn test_assert_non_empty_stderr(mut tester: TestCommand) {
    tester.assert_non_empty_stderr();
}

//checks to see if output threshold is made
fn test_expect_output_threshold(mut tester: TestCommand, test_config: &Test) {
    tester.expect_output_threshold(
        test_config
            .target_threshold()
            .expect("test did not have a threshold"),
        &test_config
            .expected()
            .expect("test did not have a expected var"),
    );
}

//checks to see if output identical
fn test_output_is_expected(mut tester: TestCommand, test_config: &Test) {
    tester.expect_output_threshold(
        1.0,
        &test_config
            .expected()
            .expect("test did not have a expected var"),
    );
}


use path_abs::{PathDir, PathFile};



pub fn broker_test(test: &Test, testdir:&str) {
    let s = test.test_type().unwrap();
    let s_slice: &str = &s[..];
    println!("{}",testdir.to_string());
    let lib = PathFile::new(testdir).unwrap();
    println!("{}",lib.to_string());
    let src = lib.parent_dir().unwrap().to_string() + "/" + &test.test_directory().unwrap();
    let test_dir = src;
    println!("{}  ",test_dir);
    let mut test_command = setup_command(&test.cmd().unwrap(), test_dir);
    match s_slice {
        "test_assert_err" => test_assert_err(test_command),
        "test_assert_exit_code" => test_assert_exit_code(test_command, test),
        "test_assert_non_empty_stderr" => test_assert_non_empty_stderr(test_command),
        "test_expect_output_threshold" => test_expect_output_threshold(test_command, test),
        "test_output_is_expected" => test_output_is_expected(test_command, test),
        _ => panic!("test case unsupported")
    }
}


/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct TestDoc {
    pub test_file_description: Option<String>,
    pub test: Option<Vec<Test>>,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize)]
pub struct Test {
    description: Option<String>,
    test_type: Option<String>,
    test_directory: Option<String>,
    exit_code: Option<i32>,
    target_threshold: Option<f64>,
    cmd: Option<String>,
    expected: Option<String>,
    points: Option<u64>,
}

impl Test {
    // Immutable access.
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn test_type(&self) -> Option<String> {
        self.test_type.clone()
    }
    pub fn test_directory(&self) -> Option<String> {
        self.test_directory.clone()
    }
    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code.clone()
    }
    pub fn target_threshold(&self) -> Option<f64> {
        self.target_threshold.clone()
    }
    pub fn cmd(&self) -> Option<String> {
        self.cmd.clone()
    }
    pub fn expected(&self) -> Option<String> {
        self.expected.clone()
    }
    pub fn points(&self) -> Option<u64> {
        self.points.clone()
    }
}

pub fn run_test_file(filename: String)->Vec<u64>{
   let contents = match fs::read_to_string(&filename) {
        Ok(contents) => contents,
        Err(_) => panic!("file does not exist"),
    };

    let decoded: TestDoc = toml::from_str(&contents).unwrap();
    //println!("{:#?}", decoded);
    let tests = decoded.test.unwrap();
    let mut v = Vec::new();
    for test in tests.iter() {
        //println!("{:?}", test);
        let result = panic::catch_unwind(|| {
            broker_test(test,&filename);
        });
        if result.is_err() {
            v.push(0);
        } 
        if result.is_ok(){
            v.push(test.points().unwrap());
        }
    }
    v
}
 
 #[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    // #[test]
    // fn test_example_parsing() {
    //     let filename = "example/test.toml";
    //     let scores = run_test_file(filename.to_string());
    //     for s in scores{
    //         assert_ne!(s,0);
    //     }
    // }
    #[test]
    fn test_percentage_diff() {
        let str1 = "1234";
        let str2 = "12345";
        let (percentage, outdiff) = percentage_diff(str1,str2);
        println!("{} {}",percentage,outdiff);
        assert!(percentage == 0.8);
    }
    #[test]
    fn test_percentage_diff2() {
        let str1 = "12345";
        let str2 = "1234";
        let (percentage, outdiff) = percentage_diff(str1,str2);
        println!("{} {}",percentage,outdiff);
        assert!(percentage == 0.8);
    }

}