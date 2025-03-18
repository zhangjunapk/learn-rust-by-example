#![allow(overflowing_literals)]
pub fn main() {
    let decimal = 65.4154_f32;

    //不允许隐式类型转换
    //let integer:u8=decimal;

    //显式类型转换
    let integer = decimal as u8;
    let character = integer as char;

    //只有u8可以直接显式转换为char
    //let character=decimal as char;
    println!(
        "类型转换,decimal:{},integer:{},character:{}",
        decimal, integer, character
    );

    println!("1000转换为u16:{}", 1000 as u16);

    //这里向下位数转型，可以直接进行mod运算来算出结果(正数情况)
    //2的8次方=256
    //1000 mod 256 =232
    println!("1000转换为u8:{}", 1000 as u8);
    //2的8次方=256
    //-1 + 256 =255
    println!("-1转换为u8:{}", -1i8 as u8);
    println!("1000 mod 256 运算 :{}", 1000 % 256);

    println!("128转换i16:{}", 128 as i16);
    //i8 -128 - 127
    println!("i8最小值:{}", i8::min_value());
    println!("i8最大值:{}", i8::max_value());
    // 在有符号的类型中，最高位1表示负数
    //128转换为二进制为 1000 0000
    //这个就是负数了，会进行补码(取反+1)
    // 0111 1111
    // 1000 0000
    // =-128
    println!("128 转换i8:{}", 128 as i8);
    //位数超出，直接从高位截断
    println!("1000转换为u8:{}", 1000 as u8);
    //位数没有超出
    //先不管符号位，再取反+1 =-24
    //232 = 1110 1000
    // 0001 0111
    // 00011000
    // -24

    println!("232转换为i8:{}", 232 as i8);

    //浮点数如果大于上限，直接饱和到最大值
    println!("300.0 转换为u8是:{}", 300.0_f32 as u8);

    println!("-100转换为u8:{}",-100.0_f32 as u8);

    println!("Nan 转换为u8:{}",f32::NAN as u8);

    unsafe{
        println!("300.0转换为u8:{}",300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 转换为u8是:{}",(-100.0_f32).to_int_unchecked::<u8>());
        println!("Nan 转换为u8是:{}",f32::NAN.to_int_unchecked::<u8>());
    }

}
