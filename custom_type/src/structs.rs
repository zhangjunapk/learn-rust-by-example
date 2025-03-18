#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        (self.top_left.x - self.bottom_right.x).abs()
            * (self.top_left.y - self.bottom_right.y).abs()
    }
    fn square(&self, left_top: Point, width: f32) -> Rectangle {
        Rectangle {
            top_left: Point {
                x: left_top.x,
                y: left_top.y,
            },
            bottom_right: Point {
                x: left_top.x + width,
                y: left_top.y + width,
            },
        }
    }
}

pub fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);

    let point: Point = Point { x: 5.2, y: 5.6 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    println!("点的坐标:({},{})", point.x, point.y);
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };
    println!("第二个坐标的点:({},{})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    println!("矩形:{:#?}",_rectangle);
    
    let area = _rectangle.rect_area();
    println!("矩形面积:{}", area);

    let new_rectangle = _rectangle.square(point, 50f32);
    println!("新的矩形:{:#?}", new_rectangle);
    let new_area = new_rectangle.rect_area();
    println!("新的矩形面积:{}", new_area);

    let _unit = Unit;
    let pair = Pair(1, 0.1);

    println!("pair包含{}和{}", pair.0, pair.1);
    //解构元组
    let Pair(integer, decimal) = pair;
    println!("pair包含{}和{}", integer, decimal);
}
