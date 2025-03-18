pub mod macros_designators;
pub mod macros_overload;
pub mod macros_repeat;
pub mod macros_dry;
pub mod macros_dsl;
pub mod macros_variadics;

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
