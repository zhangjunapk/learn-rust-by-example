use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("{}", t)
}

fn compare_types<T: Debug, U: Display>(t: &T, u: &U) {
    println!("{:?}", t);
    println!("{}", u);
}

pub fn main() {
    let string = "world";
    let i32_array = [1, 2, 3, 4, 5];
    let vec = vec![6, 7, 8, 9];
    compare_prints(&string);
    compare_types(&i32_array, &string);
    //vec没有实现Debug+Display
    // compare_prints(&vec);
}
