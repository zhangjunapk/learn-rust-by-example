use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

//给Circle实现特性，让其能通过字符串来转换
impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(_) => Err(()),
        }
    }
}

pub fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("add:{}", parsed + turbo_parsed);

    let a: Circle = "50".parse().unwrap();
    println!("{:#?}", a);
}
