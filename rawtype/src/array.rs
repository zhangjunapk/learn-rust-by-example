use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("切片中第一个元素:{}", slice[0]);
    println!("切片有:{}个元素", slice.len());
}
pub fn main(){
    let xs:[i32;5]=[1,2,3,4,5];
    let ys:[i32;500]=[0;500];
    println!("第一个元素:{}",xs[0]);
    println!("第二个元素:{}",xs[1]);

    println!("数组中的数量:{}",xs.len());

    //byte 字节大小
    println!("数组占用内存大小:{}",mem::size_of_val(&xs));
    analyze_slice(&xs);

    println!("借用数组一部分作为切片:");
    analyze_slice(&xs[1..4]);

    let empty_array:[u32;0]=[];
    assert_eq!(empty_array,[]);
    assert_eq!(&empty_array,&[][..]);

    for i in 0..xs.len()+1{
        match xs.get(i){
            None => {
                panic!("not fount element");
            }
            Some(value) => {
                println!("got element:{}",value);
            }
        }
    }
}