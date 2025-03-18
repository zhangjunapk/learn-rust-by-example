macro_rules! crate_function {
    //ident用于变量/函数名
    ($func_name:ident) => {
        fn $func_name() {
            println!("调用了{:?}()", stringify!($func_name));
        }
    };
}
crate_function!(foo);
crate_function!(bar);
macro_rules! print_result {
    //expr 用于表达式
    ($expression:expr) => {
        //这里直接转换为字符串
        println!("{:?}",stringify!($expression));
        //表达式能直接执行，获取结果
        println!("{:?}",$expression);
        println!("{:?}={:?}", stringify!($expression), $expression);
    };
}
pub fn main() {
    foo();
    bar();
    //这里调用宏，传递的就是表达式
    print_result!(1u32 + 1);
    //这里调用宏，传递的就是表达式
    print_result!({
        let x = 1u32;
        //这里是表达式
        x * x + 2 * x - 1
    });
}
