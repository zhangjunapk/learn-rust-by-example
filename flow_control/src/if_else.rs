pub fn main() {
    let n = 5;
    if (n > 0) {
        println!("{},n是正数", n);
    } else if (n < 0) {
        println!("{},n是负数", n);
    } else {
        println!("{}是0", n)
    }
    let big_n = if n < 10 {
        println!("是一个小的数，需要扩大");
        n * 10
    } else {
        println!("是一个大的数，需要缩小");
        n / 2
    };
    println!("{} -> {}", n, big_n);
}
