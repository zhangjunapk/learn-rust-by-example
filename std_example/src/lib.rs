pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod box_example;
pub mod vec_example;
pub mod strings;
pub mod options;
pub mod results;
pub mod results_q_mark;
pub mod panics;
pub mod hash;
pub mod custom_type_hash;
pub mod hashset_example;
pub mod rc_example;
pub mod arc_example;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
