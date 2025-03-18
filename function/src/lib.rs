#![feature(unboxed_closures)]

pub mod methods;
pub mod closures;
pub mod capture;
pub mod closures_input;
pub mod closures_anonymity;
pub mod funinput;
pub mod closure_output;
pub mod iter_any;
pub mod iter_find;
pub mod hof;
pub mod diverging;

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
