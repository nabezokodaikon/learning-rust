use self::users::dsl::*;
use diesel::prelude::*;

#[macro_use]
extern crate diesel;

#[derive(Debug, Queryable)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

table! {
    users(id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

fn main() {
    let url = "sample.db";
    let cn = SqliteConnection::establish(url).expect("error: cannot connecting.");
    let result = users.filter(users::age.gt(15)).load::<User>(&cn).unwrap();
    println!("result.len is {}", result.len());
    for it in result {
        println!("{:?}", it);
    }
}
