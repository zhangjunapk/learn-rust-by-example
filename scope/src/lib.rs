pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod raii;
pub mod scope_move;
pub mod move_mut;
pub mod partial_move;
pub mod borrow;
pub mod borrow_mut;
pub mod borrow_alias;
pub mod borrow_ref;
pub mod lifetime_explicit;
pub mod lifetime_fn;
pub mod lifetime_methods;
pub mod lifetime_struct;
pub mod lifetime_trait;
pub mod lifetime_bounds;
pub mod lifetime_coercion;
pub mod lifetime_static;
pub mod lifetime_elision;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
