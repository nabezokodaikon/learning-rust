fn main() {
    let f1 = 10.0f64;
    let f2 = -1_234_56f64;
    let f3 = 578.6E+77f64;
    assert_eq!(f1.ceil(), 10.0);
    assert_eq!(f2.max(f3), f3);
}
