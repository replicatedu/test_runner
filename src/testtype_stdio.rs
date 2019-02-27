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
