fn main() {
    call_closure();
    use_return_of_closure();
    use_map();

    let double = |x| x * 2;
    let triple = |x| x * 3;
    let a = call_with_one(100, double);
    let b = call_with_one(100, triple);
    println!("a is {}", a);
    println!("b is {}", b);

    let v = vec![1, 2, 3, 4, 5];
    let a = call_with_vec(&v, |i| i * 2);
    println!("a is {}", a);
}

fn call_with_vec<F>(v: &Vec<usize>, func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    v.iter().map(|&i| func(i)).sum()
}

fn call_with_one<F>(x: usize, func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(x)
}

fn call_closure() {
    let print_name = |name, age| {
        println!("name: {}, age: {}", name, age);
    };
    println!("call closure");
    print_name("masuda", 50);
}

fn use_return_of_closure() {
    println!("use_return_of_closure");
    let format_name = |name, age| format!("name: {}, age: {}", name, age);
    let x = format_name("kato", 40);
    println!("x is {}", x);
}

fn use_map() {
    println!("use map");
    let a = [("masuda", 50), ("kato", 40), ("yamada", 30)];
    let b = a
        .iter()
        .map(|(name, age)| format!("name: {}, age: {}", name, age));
    b.for_each(|i| {
        println!("{}", i);
    });
}
