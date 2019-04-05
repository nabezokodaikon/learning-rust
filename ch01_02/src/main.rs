fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    // println!("My name is {0}, {1} {0}", "Bond");

    #[derive(Debug)]
    struct Structure(i32);

    impl std::fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{:?}` won't print...", Structure(3));
    println!("This struct `{}` won't print...", Structure(3));

    println!("Pi is rough ly {:.*}", 3, 22. / 7.);
}
