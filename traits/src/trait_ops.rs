use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

//这里表示给Foo实现Add方法，操作符重载
//使其能够和Bar相加
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    //这里的Self::Output表示上面定义的类型Output
    fn add(self, rhs: Bar) -> Self::Output {
        println!("调用了Foo.add(Bar)");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, rhs: Foo) -> Self::Output {
        println!("调用了Bar.add(Foo)");
        BarFoo
    }
}

pub fn main() {
    println!("Foo+Bar={:?}", Foo + Bar);
    println!("Bar+Foo={:?}", Bar + Foo);
}
