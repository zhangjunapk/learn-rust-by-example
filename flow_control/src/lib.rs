pub mod if_else;
pub mod loop_control;
pub mod nested;
pub mod loop_return;
pub mod while_control;
pub mod for_loop;
pub mod match_flow;
pub mod destructuring_tuple;
pub mod destructuring_array;
pub mod destructuring_enum;
pub mod destructuring_ref;
pub mod destructuring_struct;
pub mod guard;
pub mod binding;
pub mod if_let;
pub mod let_else;
pub mod loop_match;

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
