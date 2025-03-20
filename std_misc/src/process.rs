use std::io::stdin;
use std::process::Command;

pub fn main() {

    let mut line = String::new();
    loop {
        stdin().read_line(&mut line);
        command(line.as_str());
    }
}

fn command(line: &str) {
    let output = Command::new(line.trim())
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if (output.status.success()) {
        println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    } else {
        println!("执行遇到错误:{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn command_test() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if (output.status.success()) {
        println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    } else {
        println!("执行遇到错误:{}", String::from_utf8_lossy(&output.stderr));
    }
}
