use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("rust.txt")?;
    file.write_all(b"Documentation du langage Rust!")?;
    Ok(())
}