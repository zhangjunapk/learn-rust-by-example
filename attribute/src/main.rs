#!allow(unused_variables)
use attribute::cfg;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//外部属性，允许未使用的变量

fn main() {
    let x = 3;

    cfg::main();
}