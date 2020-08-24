fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "sample.txt";
    let data = std::fs::read_to_string(path)?;
    println!("data is {}", data);
    Ok(())
}
