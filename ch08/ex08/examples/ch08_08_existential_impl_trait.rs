fn to_n(n: i32) -> impl Iterator {
    0..n
}

use std::iter::Filter;
use std::ops::Range;
// fn to_n_even(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
// (0..n).filter(|i| i % 2 == 0)
// }
fn to_n_even(n: i32) -> impl Iterator {
    (0..n).filter(|i| i % 2 == 0)
}

use std::fmt;
fn one() -> impl fmt::Display {
    1i32
}

// fn one_2(is_float: bool) -> impl fmt::Display {
// if is_float {
// 1.0f32
// } else {
// 1i32
// }
// }

fn gen_counter(init: i32) -> Box<dyn FnMut() -> i32> {
    let mut n = init;
    Box::new(move || {
        let ret = n;
        n += 1;
        ret
    })
}

fn gen_counter_2(init: i32) -> impl FnMut() -> i32 {
    let mut n = init;
    move || {
        let ret = n;
        n += 1;
        ret
    }
}

fn main() {
    let n = one();
    // println!("{}", n + n);
    // std::fmt::Display;
}
