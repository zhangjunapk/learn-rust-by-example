#![doc(html_playground_url = "https://play.rust-lang.org/")]


#[doc(no_inline)]
pub use animal::Dog;

/*#[doc(no_inline)]
pub use crate::mem::drop;


#[doc(hidden)]
pub use self::async_await::*;

*/
pub mod animal;
pub mod person;

pub fn add(left: u64, right: u64) -> u64 {
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
