use std::string::ToString;

fn stringfy<T: ToString>(t: T) -> String {
    t.to_string()
}

// fn stringfy(t: Box<dyn ToString>) -> String {
// t.to_string()
// }

fn stringfy_i32(t: i32) -> String {
    <i32 as ToString>::to_string(&t)
}

fn stringfy_u64(t: u64) -> String {
    <u64 as ToString>::to_string(&t)
}

fn main() {
    stringfy(1i32);
    stringfy::<i32>(1i32);
    stringfy_i32(1i32);
    stringfy(1i32);
    stringfy(1u64);
    stringfy_i32(1i32);
    stringfy_u64(1u64);

    use std::fmt::Display;
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);
}
