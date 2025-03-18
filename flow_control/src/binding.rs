fn age() -> u32 {
    15
}

pub fn main() {
    println!("你是什么类型的人");

    match age() {
        0 => println!("还没过第一个生日"),
        1 => println!(""),
        //@匹配并且绑定到变量
        n @ 1 => println!(""),
        n @ 2..=12 => println!("儿童，{}岁", n),
        n @ 13..=19 => println!("青少年:{}", n),
        n => println!("其他情况"),
    }

    match some_number() {
        Some(n @ 42) => {println!("达到42岁了")},
        Some(n) => println!("其他年龄不感兴趣"),
        _ => (),
    }



    let level = 8;
    match level {
        n @ 1..=10 => println!("新手，经验加成:{}", n * 10),
        n @ 11..=20 => println!("中级，经验加成:{}", n * 20),
        _ => println!("高级玩家"),
    }
    println!("level:{}",level);

}

fn some_number() -> Option<u32> {
    Some(42)
}


