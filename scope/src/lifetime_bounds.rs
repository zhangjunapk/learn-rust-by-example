use std::fmt::Debug;

//这里表示定义一个结构体，
// 这个Ref的生命周期至少和入参t的生命周期一致
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("{:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("{:?}", t);
}
pub fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(&x);
}
