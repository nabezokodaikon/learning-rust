#[derive(Debug, PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

enum Month {
    January = 1,
    February = 2,
    Match = 3,
    December = 12,
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGET");
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Friday);
    say_something(Weekday::Monday);

    assert_eq!(3, Month::Match as isize)
}
