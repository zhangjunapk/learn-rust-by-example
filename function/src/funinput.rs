fn call_me_closure<F>(f: F) ->i32
where
    F: Fn()->i32,
{
    f()
}

fn function() -> i32 {
    println!("我是函数");
    6
}

fn call_me<F: Fn() -> i32>(f: F) -> i32 {
    f()
}

pub fn main() {
    let closure = || {
        println!("我是闭包");
        9
    };
    call_me_closure(closure);
    println!("函数调用结果:{}", call_me(function));
    println!("闭包调用结果:{}", call_me(closure));
}

//无参数函数实现了Fn<()>
//所以上面都能成功调用
/*impl Fn<()> for fn() {
    extern "rust-call" fn call(&self, _args: ()) {
        function(); // 静态调用
    }
}*/
