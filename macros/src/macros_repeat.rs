macro_rules! find_min {
    ($x:expr) => {$x};
    //这个定义会先把参数的第一个赋值给x，再把其他的值都赋值给y
    //先递归从后面不断获取到最小的数值，然后再和第一个数值进行比较
    ($x:expr,$($y:expr),+)=>{
        std::cmp::min($x,find_min!($($y),+))
    }
}

pub fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1, 2, 3, 4));
    println!("{}", find_min!(4, 5, 6, 7, 8));
}
