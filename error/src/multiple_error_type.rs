fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}
pub fn main() {
    let numbers = vec!["1", "2", "3", "4"];
    // let empty = vec![];
    let strings = vec!["one", "two", "three"];
    println!("正常解析数字内容的字符串数组:{}", double_first(numbers));
    //println!("解析空数组:{}", double_first(empty));
    println!("解析文本内容的文本数组:{:?}", double_first(strings));
}
