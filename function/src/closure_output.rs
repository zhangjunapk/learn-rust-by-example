use std::mem;

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is : {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let mut text = "FnMut".to_owned();
    move || {
        text.push_str(" mut");
        println!("This is : {}", text)
    }
}

fn create_fnonce() -> impl FnOnce() {
    let text = "Fn_once".to_owned();
    move || {
        mem::drop(text);
    }
}

pub fn main() {
    let create_fn = create_fn();
    //实例化函数，为什么还要用mut接收
    let mut fnmut = create_fnmut();
    let fnonce = create_fnonce();

    create_fn();
    fnmut();
    fnonce(); //调用后就被销毁，无法再次调用
}
