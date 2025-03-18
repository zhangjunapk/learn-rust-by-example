use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let first = first.parse::<i32>()?; //如果结果是Ok，会直接给值，如果是error，会直接返回err
    let second = second.parse::<i32>()?;
    Ok(first * second)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(val) => {
            println!("计算出结果:{}", val)
        }
        Err(err) => {
            println!("得到错误:{:?}", err)
        }
    }
}
pub fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
