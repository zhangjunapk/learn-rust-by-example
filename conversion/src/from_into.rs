use std::convert::From;

#[derive(Debug)]
struct Flo {
    value: i32,
}

#[derive(Debug)]
struct Number {
    value: i32,
}
//给Number实现i32到自己的转换
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

impl Into<Number> for Flo {
    fn into(self) -> Number {
        Number { value: self.value }
    }
}

pub fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(20);
    println!("My number is {:#?}", num);

    let flo = Flo { value: 20 };
    let n: Number = flo.into();
    let a: Number = 1i32.into();

    //孙悟空，构建者，可以双向转换
    let sun = MonkeySun {};
    let mouse: Mouse = sun.into();
    let sun = MonkeySun::from(mouse);

    //猪八戒，使用者，只能单向into
    let pig = Pigwuneng {};
    let mouse: Mouse = pig.into();
}
// impl From<a> for b
// b::from(a)
// b=a.into()

//impl Into<a> for b
//a=b.into()

#[derive(Debug)]
struct MonkeySun {}

#[derive(Debug)]
struct Pigwuneng {}

struct Tiger {}
struct Mouse {}

impl From<Mouse> for MonkeySun {
    fn from(value: Mouse) -> Self {
        MonkeySun {}
    }
}

impl From<MonkeySun> for Mouse {
    fn from(value: MonkeySun) -> Self {
        Mouse {}
    }
}

impl From<Tiger> for MonkeySun {
    fn from(value: Tiger) -> Self {
        MonkeySun {}
    }
}

impl Into<Mouse> for Pigwuneng {
    fn into(self) -> Mouse {
        Mouse {}
    }
}
