fn main() {
    map();
    filter();
}

fn map() {
    println!("use map");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().map(|x| x * x);
    b.for_each(|x| println!("it is {}", x));
}

fn filter() {
    println!("use filter");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().filter(|&x| x % 2 == 1);
    b.for_each(|x| println!("it is {}", x));
}
