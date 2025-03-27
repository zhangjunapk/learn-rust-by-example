

///
/// ```rust,html_playground_url = "[https://play.rust-lang.org/](https://play.rust-lang.org/)"
/// println!("打印输出，可编辑playground")
/// ```

//cargo test --doc
///人物结构体
/// 持有名称
pub struct Person {
    name: String,
}

///
///人物的结构体实现
///```
/// use meta::person::Person;
/// let person=Person::new("Alice");
/// person.hello();
/// assert!(1==1);
/// ```
impl Person {
    /// 通过名称初始化结构体
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    /// 结构体实现
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}
