use std::rc::Rc;

pub fn main() {
    let string = "rc引用的字符串对象".to_string();
    {
        let rc = Rc::new(string);
        println!("引用数量:{}", Rc::strong_count(&rc));
        {
            let clone_rc = rc.clone();
            //这个rc内部使用box来存储引用
            //这里相当于在堆里面重新开辟了空间，来保存rca的引用
            let rc_b = Rc::new(clone_rc);
            println!("rcb的引用计数:{}", Rc::strong_count(&rc_b));
            println!("引用数量:{}", Rc::strong_count(&rc));
        }
        println!("引用数量:{}", Rc::strong_count(&rc));
    }
}
