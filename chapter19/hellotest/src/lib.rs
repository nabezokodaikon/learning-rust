#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 0), 10);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_under_zero() {
        assert_eq!(add(-1, -1), 0);
    }

    #[test]
    fn test_add_double() {
        assert_eq!(add_double(1.0, 1.0), 2.0);
    }

    #[test]
    fn test_add_str() {
        assert_eq!(add_str("A", "B"), "A B");
    }

    #[test]
    fn test_not_equal() {
        assert_ne!(1 + 1, 3);
    }

    #[test]
    fn test_qual_instans() {
        let mut a = Person {
            id: 100,
            name: "masuda".to_string(),
            age: 50,
        };
        assert_eq!(a, a);
    }
}

fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    if ans < 0 {
        0
    } else {
        ans
    }
}

fn add_double(x: f32, y: f32) -> f32 {
    let ans = x + y;
    if ans < 0.0 {
        0.0
    } else {
        ans
    }
}

fn add_str(x: &str, y: &str) -> String {
    let ans = format!("{} {}", x, y);
    ans.trim().to_string()
}

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}
