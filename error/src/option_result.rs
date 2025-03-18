use std::num::ParseIntError;

//这里的option的map方法，会返回
/**
Option<U>
    where
        F: FnOnce(T) -> U,
*/
//在这个例子中，返回的就是Result<i32,ParseIntError>
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>())
}

fn double_first_exchange(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let res = vec.first().map(|first| first.parse::<i32>());

    /**
        Some(Ok(x)) => Ok(Some(x)),
                Some(Err(e)) => Err(e),
                None => Ok(None),
    */
    //Option包裹Result转化为Result包裹Option
    res.transpose()
}

pub fn main() {
    let number = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["a", "b", "c"];
    println!("打印数字数组结果:{:?}", double_first(number));
    println!("打印空数组计算结果:{:?}", double_first(empty));
    println!("打印会出错的字母数组结果:{:?}", double_first(strings));


    let number = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["a", "b", "c"];
    println!("打印数字数组结果:{:?}", double_first_exchange(number));
    println!("打印空数组计算结果:{:?}", double_first_exchange(empty));
    println!("打印会出错的字母数组结果:{:?}", double_first_exchange(strings));

}
