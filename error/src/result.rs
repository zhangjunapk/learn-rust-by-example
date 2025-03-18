fn multiply(first: &str, second: &str) -> i32 {
    //如果Result遇到错误，直接unwrap会引发panic
    let first = first.parse::<i32>().unwrap();
    let second = second.parse::<i32>().unwrap();
    first * second
}

pub fn main() {
    let value = multiply("10", "20");
    println!("{}", value);
    multiply("t", "20");
}
