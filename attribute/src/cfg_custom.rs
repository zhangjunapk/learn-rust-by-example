//使用rustc 传递条件来编译并运行
// rustc --cfg some_condition cfg_custom.rs && ./cfg_custom
#[cfg(some_condition)]
fn conditional_function() {
    println!("满足条件");
}

pub fn main() {
    conditional_function();
}
