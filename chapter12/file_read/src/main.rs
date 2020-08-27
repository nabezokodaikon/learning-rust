use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let path = "sample1.txt";
    println!("read 16 bytes by buffer");
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        println!("line is {}", line?);
    }
    Ok(())
}
