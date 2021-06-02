use std::collections::HashMap;

fn foo(txt: &str) -> Result<i32, std::num::ParseIntError> {
    let a_string = String::from(txt);
    let b = a_string.parse::<i32>()?;
    Ok(b)
}
fn main() {
    {
        let v = vec![1, 2, 3];
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let mut s = String::new();
        let i = 5;
        let five = i.to_string();
        let five = String::from("5");
    }
    {
        let txt = "1";
        let r = foo(txt);
        let r = match r {
            Ok(t) => 1,
            Err(err) => 0,
        };
        println!("{}", r);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("{}", score.unwrap());

        let score = scores.get(&team_name);
        println!("{}", score.unwrap());

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
        dbg!(scores);
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
        println!("{:?}", scores);
    }
}
