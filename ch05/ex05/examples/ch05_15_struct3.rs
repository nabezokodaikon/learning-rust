struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

fn main() {
    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));
    assert_eq!((triangle.1).0, 3);

    // let bad_user = new_user(UserName(String::from("kazuki")), now, id);
    let id = Id(400);
    let now = Timestamp(4567890123);
    let user = new_user(UserName(String::from("kazuki")), id, now);

    let uv1 = UniqueValue;
    let uv2 = UniqueValue;
    assert_eq!(uv1, uv2);
}

struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

#[derive(Debug, PartialEq)]
struct UniqueValue;
