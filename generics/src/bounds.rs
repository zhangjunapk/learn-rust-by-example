use std::fmt::{Debug, Display};

fn printer<T>(t: T)
where
    T: Display,
{
    println!("{}", t);
}

struct S<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        &self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}
//这里是特性约束，让实现了指定特性的不可变引用能够作为参数
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}
//特性约束，实现了指定特性的不可变引用能够作为参数
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    let a = area(&rectangle);
    printer(a);
}
