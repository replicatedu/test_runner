use std::fmt;

#[macro_use]
extern crate toml;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Test{
    complition: String,
    execution: String,
}

#[derive(Deserialize)]
struct FileTest {
    execution_arguments: String,
    solution_output_file: String,
    student_output_file: String,
}

#[derive(Deserialize)]
struct StdioTest {
    execution_arguments: String,
    stdin: String,
    expected_stdout: String,
}

#[derive(Deserialize)]
struct CustomTest {
    execution_arguments: String,
    evaluation_command: String,
}

//cargo test -- -Z unstable-options --format=json

macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (func, input, expected) = $value;
            assert!(func(input,expected));
        }
    )*
    }
}

fn eqq(v1:u32,v2:u32)-> bool {
    v1 == v2
}

fib_tests! {
    fib_0: (eqq, 0, 0),
    fib_1: (eqq, 0, 1),
    fib_2: (eqq, 0, 0),
 }