enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

pub fn main() {
    let temp = Temperature::Celsius(30);
    match temp {
        Temperature::Celsius(t) if t > 30 => {
            println!("rock!");
        }
        Temperature::Celsius(t) => {}
        Temperature::Fahrenheit(t) if t > 86 => {}
        Temperature::Fahrenheit(t) => {}
    }

    let number: u8 = 4;
    match number {
        //这里不检查守卫的条件
        //如果编译器检查守卫条件，会增加编译成本，增加编译器复杂程度
        //因为守卫条件还能调用函数
        //所以这里就需要一个_来处理未匹配的情况
        i if i == 0 => println!("零"),
        i if i > 0 => println!("大于灵"),
        //这里不算多余
        _ => {}
    }

    let n: u8 = 6;
}
