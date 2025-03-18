use std::str::FromStr;

pub fn main() {
    let str = "60 我是内容";
    let (c, i) = get_count_item(str);
    println!("c={}, i={}", c, i);

    let mut it = str.split(" ");
    let (count_str, item) = match ((it.next(), it.next())) {
        (Some(count), Some(item)) => (count, item),
        _ => panic!(""),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("not a number");
    };

}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(" ");
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("数据错误");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("数据解析错误");
    };
    (count, item)
}
