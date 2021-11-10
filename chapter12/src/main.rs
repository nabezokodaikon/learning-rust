use std::{fs::File, io::ErrorKind};

fn hoge() -> Option<i32> {
    let a = Some(10);
    let b = a?;
    Some(b)
}

fn hoge2() -> Option<i32> {
    let a = None;
    let b = a?;
    Some(b)
}

fn main() {
    // let f = File::open("hello.txt");
    // let f = match f {
    // Ok(file) => file,
    // Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    // Ok(fc) => fc,
    // Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    // },
    // Err(error) => {
    // panic!("There was a problem opening the file: {:?}", error)
    // }
    // };
    {}
}
