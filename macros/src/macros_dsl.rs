//这里直接宏定义，计算
macro_rules! calculate {
    (eval $e:expr) => {
        println!("{}={}", stringify!($e), $e);
    };
}
pub fn main() {
    calculate! {
        eval 1+2
    }

    calculate! {
        eval 5+9
    }
}
