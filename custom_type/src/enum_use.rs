#![allow(dead_code)]

enum Stage{
    Beginner,
    Advanced,
}
enum Role{
    Student,
    Teacher,
}
pub fn main(){
    use crate::enum_use::Stage::{Beginner, Advanced};
    use crate::enum_use::Role::*;

    let stage=Beginner;
    let role=Student;

    match stage{
        Beginner=>{
            println!("你才刚开始，后面的路还很长");
        }
        Advanced=>{
            println!("你已经进入正轨了，需要学习一些更有深度的知识");
        }
    }
    match role{
        Student=>{
            println!("学生哇，真好");
        }
        Teacher=>{
            println!("老师啊，好啊");
        }
    }
}