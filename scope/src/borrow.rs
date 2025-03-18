//所有权转移
fn eat_box_i2(box_i32: Box<i32>) {
    println!("{}", box_i32);
}

//借用
fn borrow(box_i32: &i32) {
    println!("{}", box_i32);
}

pub fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6i32;
    borrow(&boxed_i32);
    borrow(&stacked_i32);
    {
        //这里被销毁后，外部不能再借用了
        // eat_box_i2(boxed_i32);
        borrow(&boxed_i32);
    }
    eat_box_i2(boxed_i32);
}
