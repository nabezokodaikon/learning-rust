fn main() {
    println!("hello `{:8}` world", "rust");
    println!("hello `{:<8}` world", "rust");
    println!("hello `{:>8}` world", "rust");
    println!("hello `{:^8}` world", "rust");
    println!("hello `{:8}` world", 123);
    println!("hello `{:<8}` world", 123);
    println!("hello `{:>8}` world", 123);
    println!("hello `{:^8}` world", 123);
}
