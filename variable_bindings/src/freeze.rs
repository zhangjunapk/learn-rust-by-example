pub fn main(){
    let mut _mutable_integer=7i32;
    {
        let _mutable_integer=_mutable_integer;
        //在这个作用域中重新定义为不可变了
        //_mutable_integer=6;
    }
    _mutable_integer = 3;
    println!("修改数据后:{}", _mutable_integer);
}