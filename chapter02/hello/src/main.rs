fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }
}
