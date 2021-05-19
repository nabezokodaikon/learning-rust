enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let sume_number = Some(5);
    let sume_string = Some("a string");
    let absent_number: std::option::Option<i32> = None;
}
