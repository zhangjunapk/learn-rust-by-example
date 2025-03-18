use std::fmt;
use std::fmt::Formatter;

struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mm({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x:{},y:{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}", self.real, self.imag)
    }
}

pub fn main() {
    println!("{}", Structure(2));

    let minmax = MinMax(20, 368);
    println!("Compare structures:");
    println!("Display:{}", minmax);
    println!("Debug:{:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "big range:{big},small_range:{small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D { x: 3.3, y: 7.6 };
    println!("Compare points:");
    println!("Display:{}", point);
    println!("Debug:{:?}", point);


    let comp=Complex{real:3.3,imag:7.2};
    println!("Display:{}",comp);
    println!("Debug:{:?}",comp);
}
