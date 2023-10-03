#![recursion_limit = "256"]
mod client;
mod types;
mod enums;

pub use client::*;
pub use types::*;
pub use enums::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
