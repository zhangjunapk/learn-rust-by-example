fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

//函数接收不可变引用闭包参数
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    //3传入不可变引用闭包
    f(3)
}

pub fn main() {
    use std::mem;
    let greeting = "hello";

    //字符串字面值所有权转移
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        mem::drop(farewell);
    };
    apply(diary);

    // println!("{}",farewell) 这里所有权被转移了
    //因为在闭包内部修改了值

    //定义闭包
    let compute_double = |x| 2 * x;
    //调用函数，传递闭包
    println!("compute 2*3 = {}", apply_to_3(compute_double));
}
