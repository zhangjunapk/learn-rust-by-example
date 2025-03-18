pub fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("x字节大小:{}", std::mem::size_of_val(&x));
    println!("y字节大小:{}", std::mem::size_of_val(&y));
    println!("z字节大小:{}", std::mem::size_of_val(&z));
    println!("i字节大小:{}", std::mem::size_of_val(&i));
    println!("f字节大小:{}", std::mem::size_of_val(&f));
}
