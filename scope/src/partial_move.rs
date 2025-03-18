pub fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    //这里不能自己实现drop，不然部分引用解构会所有权全部转移
    /*impl Drop for Person {
        fn drop(&mut self) {
            println!("Dropping Person with name: {:?}", self);
        }
    }*/
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(5),
    };
    //name 所有权转移 age用引用解构
    //name所有权被转移，age通过引用，person.age所有权还在
    let Person { name, ref age } = person;
    println!("Name: {:?}, age: {:?}", name, age);
    // println!(" {:?}", person); 报错，所有权转移了
    println!("Age: {:?}", person.age);
    //报错，所有权被转移了
    // println!("Name: {:?}", person.name);
}
