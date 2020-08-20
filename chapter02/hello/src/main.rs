fn main() {
    let s = "hello rust world.";

    let a = &s[..];
    println!("a is {}", a);
    let n = s.len();
    let a = &s[0..n];
    println!("a is {}", a);
}
