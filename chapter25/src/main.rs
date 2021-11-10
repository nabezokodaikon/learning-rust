fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(2 == 2);
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }

    #[test]
    #[should_panic]
    fn it_works2() {
        panic!();
    }
}
