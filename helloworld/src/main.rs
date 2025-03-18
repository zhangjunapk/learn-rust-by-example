use helloworld::debug;
use helloworld::display;
use helloworld::format;
use helloworld::list;

fn main() {
    mine();
    debug::main();
    display::main();
    list::main();
    format::main();
}

fn mine() {
    println!("{} 天", 31);
    println!("{0},这是{1}.{1},这是{0}", "Alice", "Nancy");
    println!(
        "hi,这是我的狗狗，她叫:{dog_name},她有{house_num}个狗窝，她是{sex}的，\
    她有{bf_num}个男朋友，她的毛发是{hair_color}",
        dog_name = "whore",
        house_num = 5,
        sex = "母",
        bf_num = 3,
        hair_color = "薰衣草灰"
    );
    println!("下面开始进制转换===========================>>>>>>>>>>");
    println!("十进制:{}", 1024);
    println!("二进制(binary):{:b}", 1024);
    println!("八进制(octal):{:o}", 1024);
    println!("十六进制(hexadecimal):{:x}", 1024);
    println!(
        "{number:>5}这是右对齐，会往左边插入空格，保持指定宽度",
        number = 1
    );
    println!("{number:0>8},这是右对齐，填充0,保持宽度8", number = 9);
    println!(
        "{number:0<6}，这是左对齐，保持宽度6,不够用0填充",
        number = 6
    );
    println!("{number:0<width$}", number = 3, width = 9);
    println!("我的名字是{0},{1},{0}", "Nancy", "x");

    let number: f64 = 1.0;
    let width: usize = 5;
    //自动捕获变量，右对齐，保持5个宽度
    println!("{number:>width$}");
}
