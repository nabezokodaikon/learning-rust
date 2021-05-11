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
}
