#![allow(dead_code)]

enum Number{
    Zero,
    One,
    Two
}

enum Color {
    Red=0xff0000,
    Green=0x00ff00,
    Blue=0x0000ff
}
pub fn main(){
    println!("zero的值是:{}",Number::Zero as i32);
    println!("one的值是:{}",Number::One as i32);

    println!("颜色红色:{}",Color::Red as i32);
    println!("蓝色是:{}",Color::Blue as i32);
}