extern crate difference;
extern crate term;

use difference::{Changeset, Difference};
use std::ffi::OsStr;
use std::path::Path;
use std::process::{self, Command};

pub fn percentage_diff(got: &str, expected: &str) -> (f64, String) {
    let mut t = term::stdout().unwrap();
    let Changeset { diffs, .. } = Changeset::new(got.trim(), expected.trim(), "");
    let mut len_same:usize = 0;
    let mut len_rem:usize = 0;
    let mut outdiff:String = String::new();
    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                len_same += x.len();
                outdiff.push_str(&format!("={}", x));
            }
            Difference::Add(ref x) => {
                outdiff.push_str(&format!("+{}", x));
            }
            Difference::Rem(ref x) => {
                len_rem += x.len();
                outdiff.push_str(&format!("-{}", x));
            }
        }
    } 
    t.fg(term::color::CYAN).unwrap();
    let total = len_same+len_rem;
    let percentage:f64 = len_same as f64 / total as f64 ;

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
                "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
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
            expected_code, code,
            "\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\
            \n===== {:?} =====\n\
             expected exit code did not match\
             \nexpected: {}\
             \nfound: {}\
             \nstdout: {}\n\nstderr: {}\
             \n=====\n",
            self.cmd, expected_code, code,
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

    pub fn expect_output_threshold(&mut self, target_threshold: f64, expected:&str) {
        let o = self.cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&o.stdout);
        let (perc_diff, out) = percentage_diff(&stdout,&expected);
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
