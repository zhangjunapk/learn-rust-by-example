enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
    Other { r: u32, g: u32, b: u32, z: u32 },
}

pub fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => {
            println!("The color is RGB! r:{} g:{},b:{}", r, g, b);
        },
        Color::HSV(h, s, v) => {
            println!("The color is HSV! h:{} s:{} v:{}", h, s, v);
        },
        Color::HSL(h, s, l) => {
            println!("The color is HSL! h:{} s:{} l:{}", h, s, l);
        },
        Color::CMY(c, m, y) => {
            println!("这是cmy的颜色表示，c:{},m:{},y:{}", c, m, y);
        },
        //解构元组枚举
        Color::CMYK(c, m, y, k) => {
            println!("这是cmyk ,c:{},m:{},y:{},k:{}", c, m, y, k);
        },
        //解构 结构体枚举
        Color::Other { r, g, b, z } => {
            println!("这是什么野鸡颜色? r:{},g:{},b:{},z:{}", r, g, b, z);
        },
    }
}
