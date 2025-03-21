pub mod doc_testing;
pub mod unit_testing;

//模块对外暴露的功能
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    //这个能彩色对比
    use pretty_assertions::assert_eq;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
