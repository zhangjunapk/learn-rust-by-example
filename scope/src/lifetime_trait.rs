#[derive(Debug)]
struct Borrowed<'a>{
    x:&'a i32
}

impl<'a> Default for Borrowed<'a>{
    fn default() -> Self {
        Self{
            x:&10
        }
    }
}

pub fn main(){
    let a:Borrowed=Default::default();

    //使用turbofish 语法初始化得到实例化对象
    let b=Borrowed::<'static>::default();

    println!("{:?}", a);
}