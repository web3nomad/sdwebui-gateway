#![recursion_limit = "256"]
mod client;
mod types;
mod enums;

pub use client::*;
pub use types::*;
pub use enums::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = Client::new("http://localhost:7860");
        assert_eq!(client.origin, "http://localhost:7860");
    }
}
