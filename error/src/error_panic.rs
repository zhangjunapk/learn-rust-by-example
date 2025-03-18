fn drink(beverage: &str) {
    if beverage == "柠檬水" {
        panic!("你好歹毒，🦣，给我喝柠檬水");
    }
    println!("没错，这个就是我想要的:{}", beverage);
}

pub fn main() {
    drink("菠萝水");
    drink("橙汁");
    drink("柠檬水");
    //panic会直接中断函数程序执行
    drink("🍎苹果水");
}
