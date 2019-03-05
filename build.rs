fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("gen_tests.rs");
    let mut f = std::fs::File::create(&destination).unwrap();

    let params = &["abc", "fooboo","asdasd"];
    for p in params {
        use std::io::Write;
        write!(
            f,
            "
#[test]
fn {name}() {{
    assert!(true);
}}",
            name = p
        ).unwrap();
    }
}