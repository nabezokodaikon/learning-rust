#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod first;
pub mod second;
pub mod third;
pub mod utils;

pub enum SortOrder {
    Ascending,
    Descending,
}
