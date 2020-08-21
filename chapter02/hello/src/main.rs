fn main() {
    print!("LOOP is ");
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        print!("{} ", i);
        i += 1;
    }
    println!();
}
