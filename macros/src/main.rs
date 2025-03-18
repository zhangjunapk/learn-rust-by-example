use macros::*;
//宏定义，能展开成源代码
macro_rules! say_hello {
    //表示接收的参数为空
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
    macros_designators::main();
    macros_overload::main();
    macros_repeat::main();
    macros_dsl::main();
    macros_variadics::main();
}
