use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {

    {
        let f = File::open("hello_.txt");

        let f = match f {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                match File::create("hello_.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("Tried to create file but there was a problem: {:?}", e)
                    },
                }
            },
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error)
            }
        };
    }

    {
        let f = File::open("hello.txt").unwrap();
    }

    {
        let f = File::open("hello.txt").expect("Failed to open hello3.txt");
    }
    
    {
        let r = read_username_from_file();
        println!("{:?}", r);
    }
    
    {
        let r = read_username_from_file_2();
        println!("{:?}", r);
    }

    {
        let r = read_username_from_file_3();
        println!("{:?}", r);
    }

    {
        let f = File::open("hello.txt")?;
        println!("{:?}", f);
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello3.txt")?;
    let mut s = String::new();
    f.read_to_string(& mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}
