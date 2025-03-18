macro_rules! test {
    //调用宏时，会自动进行规则比配
    //这里第一个表达式结束后，必须要;分号结尾
    ($left:expr ;&& $right:expr) => {
        println!(
            "左边:{:?},右边:{:?},是否and:{:?}",
            stringify!($left),
            stringify!($right),
            //这里会隐式类型推断为bool
            $left && $right
        );
    };
    //这里能够进行重载，参数要求不一样
    //中间的符号不一样
    ($left:expr; || $right:expr) => {
        println!(
            "左边:{:?},右边:{:?},是否or:{:?}",
            stringify!($left),
            stringify!($right),
            //这里为隐式类型推断为bool
            $left || $right
        );
    };
}

pub fn main() {
    test!(1+1==2i32; && 5+6==11i32);
    test!(1+1==2i32; || 5+6==10i32);
}
