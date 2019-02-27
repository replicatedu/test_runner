#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;


#[derive(Deserialize)]
struct FileTest {
    execution_arguments: String,
    solution_output_file: String,
    student_output_file: String,
}