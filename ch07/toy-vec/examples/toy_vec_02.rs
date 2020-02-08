use toy_vec::ToyVec;

fn main() {
    let e: Option<&String>;
    {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        // e = v.get(1);
        // println!("{}", e.is_some());

        // match v.pop() {
        // Some(a) => println!("{}", a),
        // None => println!("None"),
        // }
        // match v.pop() {
        // Some(a) => println!("{}", a),
        // None => println!("None"),
        // }
        // match v.pop() {
        // Some(a) => println!("{}", a),
        // None => println!("None"),
        // }
    }

    // assert_eq!(e, Some(&"Budgerigar".to_string()));

    let mut v: ToyVec<usize> = ToyVec::new();
    v.push(100);
    let e = v.get(0);
    // v.push(200);
    assert_eq!(e, Some(&100));
}
