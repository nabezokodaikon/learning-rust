fn f1(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    let hello = f1("taro");
    assert_eq!(hello, "Hello taro!");
}
