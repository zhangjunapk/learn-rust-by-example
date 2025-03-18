//这里能够通过关键词进行派生，让其自动实现特性
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        //通过let进行解构
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(u32);

pub fn main() {
    let _one_second = Seconds(1);

    let foot = Inches(12);

    println!("一英尺:{:?}", foot);
    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "更小"
    } else {
        "更大"
    };
    println!("一英寸比一米{}", cmp);
}
