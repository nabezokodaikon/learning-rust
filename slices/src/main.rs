fn main() {

    {
        println!("first_word");

        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("word: {}", word);

        s.clear();
        println!("word: {}", word);
    }

    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("hello: {}", hello);
        println!("world: {}", world);
    }

    {
        println!("example 3");
        let s = String::from("hello");
        let r = &s[0..2] == &s[..2];
        println!("r: {}", r);
    }

    {
        println!("example 4");
        let s = String::from("hello");
        let len = s.len();
        let r = &s[3..len] == &s[3..];
        println!("r: {}", r);
    }

    {
        println!("example 5");
        let s = String::from("hello");
        let len = s.len();
        let r = &s[0..len] == &s[..];
        println!("r: {}", r);
    }

    {
        println!("first_word_2");
        let mut s = String::from("hello world");
        let word = first_word_2(&s);
        println!("word: {}", word);
        s.clear();
        // cannot borrow `s` as mutable because it is also borrowed as immutable
        // println!("word: {}", word);
    }

    {
        println!("second_word");
        let mut s = String::from("hello world");
        let word = second_word(&s);
        println!("word: {}", word); 
        s.clear();
    }

    {
        println!("first_word_2_b");
        let my_string = String::from("Hello world");
        let word = first_word_3(&my_string);
        println!("word: {}", word);

        let my_string_literal = "Hello world";
        let word = first_word_3(&my_string_literal[..]);
        println!("word: {}", word);

        let word = first_word_3(my_string_literal);
        println!("word: {}", word);
    }

    {
        let x = 5;
        println!("x: {}", x);
        let x = "3";
        println!("x: {}", x);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    return &s[..]
}
