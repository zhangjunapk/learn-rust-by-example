pub fn main() {
    let immutable_box = Box::new(5u32);
    println!("不可变box:{:?}", immutable_box);

    //所有权转移，数据可见性可以更改
    let mut mutable_box = immutable_box;
    println!("可变box:{:?}", mutable_box);

    *mutable_box = 10;
    println!("变化后:{}", mutable_box);
}
