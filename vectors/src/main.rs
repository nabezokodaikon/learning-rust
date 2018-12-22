fn main() {
    {
        // let v: Vec<i32> = Vec::new();
    }

    {
        let v = vec![1, 2, 3];
        for i in v.iter() {
            println!("{}", i);
        } 
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        for i in v.iter() {
            println!("{}", i);
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("{}", third);
        let third: Option<&i32> = v.get(2); 
        println!("{:?}", third);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        
        // let does_not_exit = &v[100];
        // println!("{}", does_not_exit);
        let does_not_exit = v.get(100);
        println!("{:?}", does_not_exit);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0];
        v.push(6);
        println!("{:?}", v);
        // println!("{}", first);
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }

        println!("{:?}", v);
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in row {
            match i {
                SpreadsheetCell::Int(j) => println!("{}", j),
                SpreadsheetCell::Text(j) => println!("{}", j),
                SpreadsheetCell::Float(j) => println!("{}", j),
            }
        }
    }

    {
        let mut v1 = vec![1, 2, 3];
        let v2 = v1.pop();
        println!("{:?}", v2);
    }
}
