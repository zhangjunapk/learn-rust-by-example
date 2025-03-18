struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    //超出作用域后就会自动调用
    //这里只是一个函数钩子，具体内存释放会在内部分开实现
    fn drop(&mut self) {
        println!("调用drop方法，进行释放对象");
    }
}

pub fn main() {
    let _a = Dropable { name: "a" };

    {
        //block a

        let _b = Dropable { name: "b" };

        {
            //block b
            let _c = Dropable { name: "c" };
            let _d = Dropable { name: "d" };
            println!("正在退出块b");
        }
        println!("刚刚推出了块b");
        println!("正在退出块a");
    }
    println!("刚刚退出了块a");
    drop(_a);
    println!("函数结束")
}
