use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("程序参数数量:{}", args.len());
    for x in args {
        println!("获取到程序执行参数{}", x)
    }
}
