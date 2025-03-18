//这里定义函数
//函数接收Fn不可变引用闭包

fn apply<F>(f: F)
where
    F: Fn(),
{
    //函数内部调用不可变引用闭包，无参数调用
    f()
}

//mir内部实现就像这样
/*
struct Closure<'a> {
    x: &'a i32,
}

impl FnMut<()> for Closure<'_> {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        todo!()
    }
}

impl FnOnce<()> for Closure<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        todo!()
    }
}

impl<'a> Fn<()> for Closure<'a> {
    extern "rust-call" fn call(&self, args: ()) -> Self::Output {
        println!("打印数字:{}", &self.x)
    }
}*/

pub fn main() {
    let x = 7;
    //定义闭包逻辑
    //内部定义包含变量捕获的结构体
    //并且实现fn fn_once fn_mut
    let print = || println!("{}", x);
    //通过函数调用闭包
    apply(print);

    // let s=Closure{x:&6};
    // apply(s);
}
