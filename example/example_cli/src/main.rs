use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut buffer2 = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}",&buffer);
    io::stdin().read_to_string(&mut buffer2)?;
    println!("{}",&buffer2);
    Ok(())
}