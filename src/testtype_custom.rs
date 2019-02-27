#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;


#[derive(Deserialize)]
struct CustomTest {
    execution_arguments: String,
    evaluation_command: String,
}
