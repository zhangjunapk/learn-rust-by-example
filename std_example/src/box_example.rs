use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    Box::new(origin())
}

pub fn main() {
    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 0.5, y: 0.5 },
    };
    let boxed_rectangle = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 0.5, y: 0.5 },
    });

    let point: Box<Point> = Box::new(origin());

    let multiple = Box::new(boxed_rectangle);

    //box指针，都占用8个字节
    println!("占用大小:{}", mem::size_of_val(&point));
    println!("占用大小:{}", mem::size_of_val(&rectangle));
    println!("占用大小:{}", mem::size_of_val(&multiple));
    println!("占用大小:{}", mem::size_of_val(&*multiple));
    println!("占用大小:{}", mem::size_of_val(&point));
}
