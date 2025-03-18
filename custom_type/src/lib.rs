pub mod structs;
pub mod enums;
pub mod enum_use;
pub mod eum_c_like;
pub mod enum_list;
pub mod constants;

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
