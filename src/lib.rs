//这个在应用级别能够使用，crate级别用不了
#![doc(html_playground_url = "https://playground.example.com/")]
///
/// ```
/// println!("打印内容");
/// ```
///
///

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
