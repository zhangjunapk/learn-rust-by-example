extern crate core;

use core::slice;
use unsafe_example::*;
fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        //解引用裸指针
        assert!(*raw_p == 10);
    }

    let vector = vec![1, 2, 3];

    let ptr = vector.as_ptr();
    let len = vector.len();

    unsafe {
        //这里直接通过指针和长度来获取切片
        let my_slice = slice::from_raw_parts(ptr, len);
        println!("vector: {:?}", my_slice);
        println!("raw vector: {:?}", vector);
        println!("raw pointer: {:?}", ptr);
        assert_eq!(my_slice, vector);
    }
    asm::main();
}
