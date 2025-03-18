pub fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("绑定:{}", a_binding);
    let another_binding;
    // println!("另一个绑定:{}",another_binding);
    another_binding = 1;
    println!("另一个绑定初始化:{}", another_binding);
}
