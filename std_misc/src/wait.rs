use std::process::Command;

pub fn main() {
    let mut child = Command::new("sleep").arg("10").spawn().unwrap();
    //wait函数会等待命令执行完毕
    let result = child.wait().unwrap();
    println!("{}", result);
}
