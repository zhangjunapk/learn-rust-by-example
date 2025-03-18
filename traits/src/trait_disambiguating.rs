trait UsernameWidget {
    fn get(&self) -> String;
}
trait AgeWidget {
    fn get(&self) -> u8;
}
struct Form {
    username: String,
    age: u8,
}
//这里生成的函数签名，能够区分实现的特性
//fn <Form as UsernameWidget>::get(_self: &Form) -> String {
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn main() {
    let form = Form {
        username: "kobee".to_owned(),
        age: 37,
    };
    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);
    println!("username={:?}, age={:?}", username, age);
}
