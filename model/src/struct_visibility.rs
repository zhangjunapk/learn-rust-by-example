use crate::struct_visibility::my::CloseBox;

mod my{
    pub struct OpenBox<T>{
        pub content:T
    }
    pub struct CloseBox<T>{
        content:T,
        pub num:i32
    }

    impl<T> CloseBox<T>{
        pub fn new(content:T)->CloseBox<T>{
            CloseBox{
                content:content,
                num:0
            }
        }
    }

}

pub fn main(){
    let open_box=my::OpenBox{content:90};
    println!("开放盒子:{}",open_box.content);

    // let b=my::CloseBox{content:"a"};

    let b=my::CloseBox::new("我是内容，");

    //这样也不行。必须所有字段都初始化
    // let c=CloseBox{num:1};
}