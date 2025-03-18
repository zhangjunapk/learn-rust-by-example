#![feature(specialization)]

struct GenVal<T> {
    gen_val: T,
}
/*
impl<T> GenVal<T> {
    default fn value(&self) -> &T {
        &self.gen_val
    }
}*/
//对不同字段泛型分别实现
impl GenVal<i32> {
    fn value(&self) -> String {
        self.gen_val.to_owned().to_string()
    }
}

//对&str进行分别实现
impl GenVal<&str> {
    fn value(&self) -> &str {
        &self.gen_val
    }
}

pub fn main() {
    let x = GenVal { gen_val: 5 };
    println!("{}", x.value()); // 150
    let y = GenVal { gen_val: "hello" };
    println!("{}", y.value()); // "hello"
}
