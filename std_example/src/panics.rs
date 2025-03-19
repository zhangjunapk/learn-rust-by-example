fn div(x: i32, y: i32) -> i32 {
    if (y == 0) {
        panic!("不能除以0");
    } else {
        x / y
    }
}

pub fn main() {
    // rustc panics.rs && valgrind ./panics
    //panic不会导致内存泄漏
    let a = Box::new(2u32);
    div(2, 0);
}
