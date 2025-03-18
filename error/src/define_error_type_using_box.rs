use std::error;
use std::fmt::{Debug, Display, Formatter};

//这里类型重定义
//这里把报错信息通过box包装起来
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct VecError;

impl Debug for VecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

//这里时error::Error的特性导致的，Error必须要有Display特性
impl Display for VecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "数组错误，无法解析")
    }
}

impl error::Error for VecError {}

fn first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| VecError.into())
        .and_then(|first|
            first.parse::<i32>()
                .map_err(|e| e.into())
                .map(|x| x * 2)
        )
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The number is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["a", "b"];

    print(first(numbers));
    print(first(empty));
    print(first(strings));
}
