use std::num::ParseIntError;

fn multiple(first: &str, second: &str) -> Result<i32, ParseIntError> {
    match first.parse::<i32>() {
        Ok(first) => match second.parse::<i32>() {
            Ok(second) => Ok(first * second),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    //first解析完成后，会执行and_then
    first
        .parse::<i32>()
        .and_then(|first| second.parse::<i32>().map(|second| first * second))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(res) => println!("数值运算得到结果:{}", res),
        Err(err) => println!("数值运算错误:{}", err),
    }
}

pub fn main() {
    let val = multiple(&"1", &"2");
    print(val);

    let val = multiply(&"23", &"10");
    print(val);
}
