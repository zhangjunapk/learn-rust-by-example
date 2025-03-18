fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x是:{},y是:{}", x, y);
}

fn failed_borrow<'a>() {
    //内部变量存储到栈，函数结束后被释放，因此不能指定为'a
    let _x = 22;
    //函数内定义的字段，无法转换为长周期的生命周期
    // let _y: &'a i32 = &_x;
}

pub fn main() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();
}
