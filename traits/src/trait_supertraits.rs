trait Person {
    fn name(&self) -> String;
}

//特性可以进行继承
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
//特性可以进行多继承
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
fn print(student: &dyn CompSciStudent) -> String {
    format!(
        "姓名:{},学校:{},最喜欢的语言:{},git账户:{}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct XiaoMing {}

impl Programmer for XiaoMing {
    fn fav_language(&self) -> String {
        "Rust".to_string()
    }
}

impl Student for XiaoMing {
    fn university(&self) -> String {
        "北京大学".to_string()
    }
}

impl Person for XiaoMing {
    fn name(&self) -> String {
        "小名".to_string()
    }
}

impl CompSciStudent for XiaoMing {
    fn git_username(&self) -> String {
        "zhangjun.apk@gmail.com".to_string()
    }
}

pub fn main() {
    let xiao_ming = XiaoMing {};
    println!("{}", print(&xiao_ming));
}
