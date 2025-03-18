pub mod trait_derive;
pub mod trait_dyn;
pub mod trait_ops;
pub mod trait_drop;
pub mod trait_iter;
pub mod trait_impl;
pub mod trait_clone;
pub mod trait_supertraits;
pub mod trait_disambiguating;

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
