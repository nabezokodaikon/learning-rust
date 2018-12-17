fn main() {
    {
        println!("example 1");

        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        println!("example 2");

        // let s = String::from("hello");
        // change(&s);
    }

    {
        println!("example 3");

        let mut s = String::from("hello");
        change(&mut s);
        println!("s: {}", s);
    }

    {
        println!("example 4");

        let mut s = String::from("hello");
        let r1 = &mut s;

        // cannot borrow `s` as mutable more than once at a time
        // let r2 = &mut s;
        
        println!("r1: {}", r1);
        // println!("r2: {}", r2);
    }

    {
        println!("example 5");

        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // cannot borrow `s` as immutable because it is also borrowed as mutable
            // println!("s: {}", s);
            println!("r1: {}", r1);
        }

        let r2 = &mut s;
        println!("r2: {}", r2);
    }

    {
        println!("example 6");

        let mut s = String::from("hello");
        let r1 = &s;
        println!("s: {}, r1: {}", s, r1);

        let r2 = &s;
        println!("s: {}, r2: {}", s, r2);

        // cannot borrow `s` as immutable because it is also borrowed as mutable
        // let r3 = &mut s;
        // println!("s: {}, r3: {}", s, r3);
    }

    {
        println!("example 7");
        // let r = dangle();
        let r = no_dangle();
        println!("r: {}", r);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn change(some_string: &String) {
    // some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 変数sは関数内で開放されるため、参照を返すことはできない。
// fn dangle() -> &String {
    // let s = String::from("hello");
    // &s;
// }

// 変数sの所有権を関数外へ渡すため、エラーにならない。
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
