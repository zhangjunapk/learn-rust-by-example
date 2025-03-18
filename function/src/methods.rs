#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //关联函数
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    //关联函数，不需要实例就能调用
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    //实例才能调用
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        (x1 - x2) * (y1 - y2).abs()
    }
    //实例才能调用
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    //这里的self会被直接消费掉，因为这个self表示作为引用来使用的
    fn destroy(self) {
        let Pair(f, s) = self;
        println!("正在销毁 Pair({}, {})", f, s);
    }
}

pub fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("矩形的周长:{}", rectangle.perimeter());
    println!("矩形的面积:{}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 2.0),
    };
    println!("变换之前的矩形信息{:#?}", square);
    square.translate(1.0, 1.0);
    println!("变换之后的矩形信息{:#?}", square);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // println!("pair:{:?}", pair) 直接报错，对象被move
}
