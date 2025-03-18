use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> AliasedResult<i32> {
    first
        .parse::<i32>()
        .and_then(|first| second.parse::<i32>().map(|second| first * second))
}
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(val) => {
            println!("获取到的返回值:{}", val)
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

pub fn main() {
    let val = multiply("10", "20");
    print(val);
    let val=multiply("t", "20");
    print(val);
}
