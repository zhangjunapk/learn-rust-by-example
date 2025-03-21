use std::env;

fn shutdown(arg: u8) {
    println!("shutdown函数被调用:{}", arg);
}

fn open_fire(arg: u8) {
    println!("open fire函数被调用:{}", arg);
}
fn help() {
    println!("可调用函数列表:shutdown open_fire")
}
pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("只有一个参数,不执行什么操作")
        }
        2 => {
            println!("第一个参数:{:?}", args.get(0));
            println!("第二个参数:{:?}", args.get(1));
            println!("只有两个参数，也不执行什么操作")
        }
        3 => {
            if let Some(arg) = args.get(1).and_then(|s| s.parse::<u8>().ok()) {
                if let Some(command) = args.get(2) {
                    match command.as_str() {
                        "open_fire" => open_fire(arg),
                        "shutdown" => shutdown(arg),
                        _ => help(),
                    }
                }
            }
        }
        _ => {}
    }
}
