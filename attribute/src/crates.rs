#![crate_type = "lib"]
#![crate_name = "rary"]

pub fn public_function() {
    println!("我是公共函数");
}
fn private_function() {
    println!("调用了rary的私有函数");
}

pub fn indirect_access() {
    println!("调用了indirect_access");
    private_function();
}
