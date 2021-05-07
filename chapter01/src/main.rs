fn main() {
    let mut a = 10;
    let a_mut_ref = &mut a;
    *a_mut_ref = 20;
    println!("{}", a_mut_ref);
}
