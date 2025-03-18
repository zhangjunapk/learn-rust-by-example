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
