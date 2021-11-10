fn main() {
    {
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];
        let iter = a1.iter().zip(a2.iter());
        for i in iter {
            dbg!(i);
        }
    }
    {
        let a = [1, 2, 3];
        let iter = a.iter().map(|x| 2 * x);
        for i in iter {
            dbg!(i);
        }
    }
    {
        let a = [0i32, 1, 2];
        let iter = a.iter().filter(|x| x.is_positive());
        for i in iter {
            dbg!(i);
        }
    }
    {
        let a = [1, 2, 3];
        let sum = a.iter().fold(0, |acc, x| acc + x);
        println!("{}", sum);
    }
    {
        let a = [1, 2, 3];
        let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
        dbg!(doubled);
    }
    {
        let a = ['a', 'b', 'c'];
        let iter = a.iter().enumerate();
        for i in iter {
            dbg!(i);
        }
    }
    {
        let a = [1, 4, 2, 3];
        let sum = a
            .iter()
            .cloned()
            .inspect(|x| println!("about to filter: {}", x))
            .fold(0, |sum, i| sum + i);
        println!("{}", sum);
    }
}
