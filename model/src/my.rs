mod inaccessible;
//在当前crate可见
pub(crate) mod nested;
pub fn function() {
    println!("调用了my::function");
}
fn private_function() {
    println!("调用了my::private_function")
}
pub fn indirect_access() {
    println!("调用my::indirect_access");
    private_function();
}
