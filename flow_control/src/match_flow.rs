pub fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        //这里表示1到6,1闭 6开的数值匹配
        1..6 => {
            println!("你这还是小孩子");
        }
        //这里宝石[7,17]闭区间的数值匹配
        6..=17 => {
            println!("你快长大了:{}", number);
        }
        //这里能够匹配多个数值
        18 | 19 | 20 => {
            println!("黄金一样的年纪");
        }
        _ => {
            println!("其他的年纪，你猜是什么年纪");
        }
    }
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("布尔:{},二进制:{}", boolean, binary);
}
