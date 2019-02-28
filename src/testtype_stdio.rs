#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct StdioTest {
    execution_arguments: String,
    stdin: String,
    expected_stdout: String,
}



//https://hoverbear.org/2014/09/07/command-execution-in-rust/