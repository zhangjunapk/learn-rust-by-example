fn destroy_box(c: Box<i32>) {
    println!("将被销毁box")
}

pub fn main() {
    //u32实现了copy，所有权转移后，以前的还能用
    let x = 5u32;
    let y = x;
    println!("x={} y={}", x, y);

    let a = Box::new(9i32);
    println!("a={}", a);
    let b = a;
    println!("b={}", b);
    // println!("Hello, world!{}",a);

    destroy_box(b);
    // println!("a={}", a); 所有权已经转移到b,b已经转移到函数内部所有并被清理
    // println!("a={}", b);
}
