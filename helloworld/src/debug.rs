#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn main() {
    println!("{:?}个月，在一年中", 12);
    println!(
        "{1:?} {0:?}是这个{actor:?}的名字",
        "mask",
        "elon",
        actor = "em"
    );
    println!("{:?}，打印实现了Debug特性的结构体", Structure(3));
    println!("只想要一个数字显示{:?}", Deep(Structure(7)));

    //尝试美化打印
    let name="Peter";
    let age=27;
    let peter=Person{name,age};
    println!("{:#?}",peter)
}
