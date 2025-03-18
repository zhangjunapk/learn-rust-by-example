use std::fmt;
use std::fmt::Formatter;

enum ErrorType {
    Invalid(i32, u8),
    Timeout,
}

//实际使用中，可能会采用这种定义错误的方式，
// 定义错误为枚举，并且给每个报错都实现Display，用于精确提示报错
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::Invalid(code, payload) => {
                write!(f, "Invalid code {} for payload {}", code, payload)
            }
            ErrorType::Timeout => {
                write!(f, "Timeout")
            }
        }
    }
}

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "数据无效，无法解析为数字")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        //如果option有值就返回Result
        //如果没值就返回DoubleError
        .ok_or(DoubleError)
        .and_then(|first| {
            first
                .parse::<i32>()
                //传递fnOnce闭包，如果解析错误，就执行闭包来获取返回的错误，包裹到Err枚举的元组中
                .map_err(|_| DoubleError)
                .map(|x| 2 * x)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(val) => {
            println!("获取到执行结果:{}", val);
        }
        Err(err) => {
            println!("执行出错:{}", err)
        }
    }
}

pub fn main() {
    let number = vec!["1", "2"];
    let empty = vec![];
    let strings = vec!["a", "1"];

    print(double_first(number));
    print(double_first(empty));
    print(double_first(strings));
}
