pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(0, add(0, 0));
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("expected panic");
    }
}
