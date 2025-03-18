fn crate_box(){
    let _box1=Box::new(3i32);
    // std::mem::forget(_box1);
}


struct ToDrop;
impl Drop for ToDrop{
    //离开作用域就会被调用
    fn drop(&mut self) {
        println!("drop被执行");
    }
}

pub fn main(){
    let _box2=Box::new(3i32);
    {
        let _box3=Box::new(4i32);
    }
    for _ in 0u32..1_000{
        crate_box();
    }
    //这个在栈上，方法结束就会被释放
    let x=ToDrop;
    println!("创建了ToDrop");



}