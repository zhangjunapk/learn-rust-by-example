pub mod gen_fn;
pub mod generic_impl;
pub mod a;
pub mod gen_trait;
pub mod bounds;
pub mod testcase_empty;
pub mod multi_bounds;
pub mod generic_where;
pub mod new_types;
pub mod generic_problem;
pub mod generic_types;
pub mod generic_phantom;
pub mod generic_units;

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
