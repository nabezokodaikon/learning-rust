fn main() {
    {
        let mut s = String::new();
        println!("{}", s);
    }

    {
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
        let r = data.to_string() == String::from("initial contents");
        println!("{}", r);
        
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
    }
    
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("{}", s);
    }

    {
        let s1 = String::from("Hello ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("{}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // let s = s1 + "-" + &s2 + "-" + &s3;
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }

    {
        let s1 = String::from("hello");
        // let h = s1[0];
    }

    {
        println!("{}", String::from("Hola").len());
        println!("{}", String::from("こんにちは").len());
    }

    {
        let hello = "こんにちは";
        let s = &hello[0..4];
        println!("{}", s);
    }

    {
        for c in "こんにちは".bytes() {
            println!("{}", c);
        }
    }
}
