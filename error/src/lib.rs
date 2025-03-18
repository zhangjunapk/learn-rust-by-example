#![feature(unboxed_closures)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod error_panic;
pub mod abort_unwind;
pub mod option_unwrap;
pub mod option_wrap_example;
pub mod option_unwrap_map;
pub mod option_unwrap_and_then;
pub mod option_unwrap_defaults;
pub mod result;
pub mod result_map;
pub mod result_alias;
pub mod early_returns;
pub mod result_q_mark;
pub mod multiple_error_type;
pub mod option_result;
pub mod define_error_type;
pub mod define_error_type_using_box;
pub mod option_result_q_mark;
pub mod error_wrap;
pub mod iter_result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
