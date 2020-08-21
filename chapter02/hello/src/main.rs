#[derive(Debug)]
enum LANG {
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRANCH = 33,
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x = 'C';
    let m = match x {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => -1,
    };
    println!("m is {}", m);
}
