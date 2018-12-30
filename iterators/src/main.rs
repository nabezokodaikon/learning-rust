fn example01() {
    println!("example01");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}

fn main() {
    example01();
}
