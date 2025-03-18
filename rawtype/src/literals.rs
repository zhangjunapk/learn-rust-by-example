#[allow(arithmetic_overflow)]
pub fn main() {
    println!("1+2={}", 1u32 + 2);
    //-1最后执行mod运算，得出u32最大值
    let x: u32 = 1u32.wrapping_sub(2);
    println!("1-2={}", x);

    println!("1e4 is {},-2.5e-3 is{}", 1e4, -2.5e-3);

    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    println!("0011 and 0101 is:{:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is:{:04b}", 0b0011u32 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //1左移5位
    //1
    //100000
    //32
    println!("1 <<5 is {}", 1u32 << 5);
    //右移5位,最多右移成0
    println!("1>>5 is {}", 1u32 >> 5);
    //右移
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("one million is written as {}", 1_000_000u32);
}
