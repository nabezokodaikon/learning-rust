fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    vec_change(&mut v);
    for i in v {
        print!("{} ", i);
    }
    println!();
}

fn vec_change(v: &mut Vec<i32>) {
    println!("called vec_change");
    for i in v {
        *i = *i * 10;
    }
}
