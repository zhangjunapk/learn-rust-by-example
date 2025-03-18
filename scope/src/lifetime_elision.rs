fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}
//这里两个方法的签名是一致的，上述方法生命周期由编译器推断
fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}
//这里的方法签名和上面的一致，上面的方法生命周期由编译器自动推断
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

pub fn main() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
}
