#![feature(specialization)]

struct Val {
    val: f64,
}
struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl GenVal<i32> {
    fn value(&self) -> i32 {
        &self.gen_val * 30
    }
}

//当前还不能稳定支持默认实现
/*impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}*/

pub fn main() {
    let x = Val { val: 3.3 };
    let y = GenVal { gen_val: 123 };
    println!("{}, {}", x.value(), y.value());
}
