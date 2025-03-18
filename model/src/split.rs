use crate::my;

fn function() {
    println!("调用了split::function")
}
fn main() {
    //调用my下面的公共函数
    my::function();
    //调用这个rs文件中的函数
    function();
    my::indirect_access();
    my::nested::function();
}
