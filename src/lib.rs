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
struct StdioTest {
    execution_arguments: String,
    stdin: String,
    expected_stdout: String,
}

//cargo test -- -Z unstable-options --format=json
//https://github.com/servo/html5ever/blob/master/html5ever/tests/tokenizer.rs
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

