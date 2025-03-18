use std::error;
use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct VecError;

impl Debug for VecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for VecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "数组错误，无法解析出结果")
    }
}

impl error::Error for VecError {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    //?如果是Some直接把值取出来赋值，如果报错直接抛出
    let first = vec.first().ok_or(VecError)?;
    let value = first.parse::<i32>()?;
    Ok(value * 2)
}

fn print(result: Result<i32>) {
    match result {
        Ok(value) => println!("The value is {}", value),
        Err(e) => println!("Error: {}", e),
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
