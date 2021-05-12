fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn copy_trait_check<T: Copy>(_: T) {}

fn main() {
    {
        let a: i32 = 10;
        let a_ref = &a;
        let a_ref_copy = a_ref;
        println!("{} {} {}", a, a_ref, a_ref_copy);
    }
    {
        let mut a = 10;
        println!("{}", type_of(&a));
        let a_mut_ref = &mut a;
        let a_mut_ref_move = a_mut_ref;
        println!("{}", a_mut_ref_move);
    }
    {
        // let s = String::from("hello");
        // copy_trait_check(s);
        let a = 10;
        copy_trait_check(a);
    }
    {
        let a = 42;
        let ref_ref_ref_a = &&&a;
        let ref_a = **ref_ref_ref_a;
        let b = *ref_a;
        println!("{} {}", a, b);
    }
    {
        // let a = 10;
        // let a_ref = &a;
        // let a_ref_ref = &a_ref;
        // println!("{}", a == a_ref);
        // println!("{}", a_ref_ref == a_ref);
    }
    {
        let a: i32 = 10;
        let b: u32 = 20;
        let c: f32 = 0.;
        let d: &i32 = &50;
        println!("{} {} {} {}", a, b, c, d);
    }
    {
        let (x, y, z) = (1, 2, 3);
        let [a, b, c] = [4, 5, 6];
        let (i, _, _) = (7, 8, 9);
        println!("xyz = {} {} {}", x, y, z);
        println!("abc = {} {} {}", a, b, c);
        println!("  i = {}", i);
    }
    {
        let str_len = String::from("hello world!");
        let str_len = str_len.len();
        println!("{}", str_len);
    }
    {
        let a = 10;
        {
            let mut a = 20;
            a += 30;
            println!("{}", a);
        }
        println!("{}", a);
    }
    {
        let a = 13u8;
        let b = 7u32;
        let c = a as u32 + b;
        println!("{}", c);

        let t = true;
        println!("{}", t as u8);
    }
    {
        let a = [1, 2, 3, 4, 5];
        let a_slice = &a[1..3];
        dbg!(a_slice);
    }
    {
        let mut sum = 0;
        for i in 1..10 {
            sum += i;
        }
        println!("sum = {}", sum);
    }
    {
        let s = String::from("hello");
        let slice = &s[0..2];
        println!("{}", slice);
        let slice = &s[0..=2];
        println!("{}", slice);
        let slice = &s[..2];
        println!("{}", slice);
        let slice = &s[3..s.len()];
        println!("{}", slice);
        let slice = &s[3..];
        println!("{}", slice);
        let slice = &s[..];
        println!("{}", slice);
    }
}
