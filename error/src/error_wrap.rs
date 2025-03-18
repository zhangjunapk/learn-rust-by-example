use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    VecError,
    ParseError(ParseIntError),
}

impl Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DoubleError::VecError => write!(f, "数组错误，无法获取元素"),
            DoubleError::ParseError(e) => write!(f, "元素解析错误 {}", e),
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(value: ParseIntError) -> Self {
        DoubleError::ParseError(value)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::VecError)?;
    //这里的❓，出错后会把ParseIntError转换为DoubleError
    let value = first.parse::<i32>()?;
    Ok(2*value)
}

fn print(result: Result<i32>) {
    match result {
        Ok(v) => println!("The result is {}", v),
        Err(e) => println!("遇到错误:{}", e),
    }
}

pub fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["a", "2"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
