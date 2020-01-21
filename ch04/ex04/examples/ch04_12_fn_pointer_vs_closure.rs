fn main() {
    let x = 4;

    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };
    assert!(flipflop());
    assert!(!flipflop());
    assert!(flipflop());
    assert!(state);

    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    f = |n| n * 4;
    assert_eq!(f(2), 8);

    // let x = 4;
    // f = |n| n * x;

    let mut v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);

    v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(str::len)
        .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
}
