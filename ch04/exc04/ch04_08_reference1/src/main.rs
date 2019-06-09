fn f1(mut n: u32) {
    n = 1;
    println!("f1: n = {}", n);
}

fn f2(n_prt: &mut u32) {
    println!("f2: n_prt = {:p}", n_prt);

    *n_prt = 2;
    println!("f2 *n_prt = {}", *n_prt);
}

fn main() {
    let mut n = 0;
    f1(n);
    println!("main: n = {}", n);

    f2(&mut n);
    println!("main: n = {}", n)
}
