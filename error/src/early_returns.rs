use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let first = match first.parse::<i32>() {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    println!("执行到中间");

    let second = match second.parse::<i32>() {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    Ok(first * second)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(res) => {
            println!("得到结果:{}", res)
        }
        Err(err) => {
            println!("执行错误:{:?}", err)
        }
    }
}

pub fn main() {
    print(multiply("100", "2"));
    print(multiply("t", "2"));
}
