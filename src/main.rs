use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut the_process = Command::new("echo")//.arg("foo")
        .spawn().ok().expect("Failed to execute.");
    // Get a Pipestream which implements the writer trait.
    // Scope, to ensure the borrow ends.
    let _ = {
        let the_stdin_stream = the_process.stdin.as_mut()
            .expect("Couldn't get mutable Pipestream.");
        // Write to it in binary.
        the_stdin_stream.write(b"tests")
            .ok().expect("Couldn't write to stream.");
        the_stdin_stream.write(b"Foo this, foo that!")
            .ok().expect("Couldn't write to stream.");
        // Flush the output so it ends.
        the_stdin_stream.flush()
            .ok().expect("Couldn't flush the stream.");
    };
    // Wait on output.
    match the_process.wait_with_output() {
        Ok(out)    => print!("{:?}", out),
        Err(error) => print!("{}", error)
    }

    // let output = Command::new("ls").arg("-aFl").output().unwrap().stdout;
    // let output = String::from_utf8_lossy(&output);
    // println!("First program output: {:?}", output);
    // let put_command = Command::new("echo")
    //     .stdin(Stdio::piped())
    //     .spawn()
    //     .unwrap();
    // write!(put_command.stdin.unwrap(), "{}", output).unwrap();
}
