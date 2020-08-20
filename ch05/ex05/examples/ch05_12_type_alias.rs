type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SharedMap<K, V> = Rc<RefCell<HashMap<K, V>>>;

fn main() {
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);
    let bad_user = new_user(String::from("kazuki"), now, id);
}
