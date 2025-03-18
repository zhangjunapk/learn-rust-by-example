extern crate rand;

use rand::Rng;
use std::fmt::Debug;

static NUM: i32 = 18;
//这里直接把具有static作用域的引用给返回了
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

/*fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng(); // 使用线程本地随机数生成器
    let mut boxed = Box::new([0; 100]);
    rng.fill(&mut *boxed); // 直接填充 [usize; 100]
    Box::leak(boxed)
}*/

fn rand_vec() -> &'static [i8; 100] {
    let mut boxed: Box<[i8; 100]> = Box::new([0; 100]);
    rand::fill(&mut boxed[..]);
    //这里能直接转换生命周期为函数定义的'static
    Box::leak(boxed)
}

pub fn main() {
    {
        let static_string = "存储在只读内存中";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num = 9;
        //这里直接把static引用作为更短的生命周期了,作为代码块生命周期了
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {}", NUM);

    let r;
    {
        let x = 8;
        //这里给r进行赋值，但是生命周期被缩短了，如果在外部进行使用r，会直接提示生命周期不够长
        r = &x;
    }
    //使用地方
    // println!("r: {}", r);

    let first: &'static [i8; 100] = rand_vec();
    let second: &'static [i8; 100] = rand_vec();
    println!("是否相等:{}", first == second);
    assert_ne!(first, second);


    let i=5;
    //这里的变量i存储在栈上，并且实现了Copy特性，
    //这里就是一个独立的副本，没有生命周期限制，因此能够作为'static生命周期传递进去
    print_it(i);
    //这里是不可变引用，就不满足'static生命周期的限制了
    // print_it(&i);
}

fn print_it(input: impl Debug + 'static) {
    println!("打印传入的值:{:?}", input);
}
