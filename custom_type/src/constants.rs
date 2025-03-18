static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn main() {
    let n = 16;

    println!("这是:{}", LANGUAGE);
    println!("阈值为：{}", THRESHOLD);
    println!("{}是{}", n, if is_big(n) { "大的" } else { "小的" });
}
