pub mod threads;
pub mod testcase_mapreduce;
pub mod channels;
pub mod path_example;
pub mod file_open_example;
pub mod file_create;
pub mod file_read_lines;
pub mod process;
pub mod pipe;
pub mod wait;

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
