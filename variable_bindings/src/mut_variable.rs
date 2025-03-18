pub fn main(){
    let _immutable_binding=1;
    let mut mutable_binding=2;

    println!("修改前:{}",mutable_binding);
    mutable_binding+=1;
    println!("修改后:{}",mutable_binding);

    //_immutable_binding+=1; 报错，不可变
}