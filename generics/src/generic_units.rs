use std::marker::PhantomData;
use std::ops::Add;
#[derive(Debug, Copy, Clone)]
enum Inch {}
#[derive(Debug, Copy, Clone)]
enum Mm {}
#[derive(Debug, Copy, Clone)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

pub fn main() {
    //这里把枚举作为泛型，用于标记类型
    let a: Length<Inch> = Length(10.00, PhantomData);
    let b: Length<Mm> = Length(1200.0, PhantomData);
    let two_a = a + a;
    let two_b = b + b;
    println!("一英寸+一英寸={:?}英寸", two_a);
    println!("一米+一米={:?}米", two_b);
}
