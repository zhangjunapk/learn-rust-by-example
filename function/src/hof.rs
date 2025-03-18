fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn main() {
    println!("找出所有平方为🤹且小于1000的数字之和");
    let upper = 1000;
    let mut acc = 0;
    for n in 0..upper {
        let n_squared = n * n;
        if (n_squared >= upper) {
            break;
        } else if is_odd(n) {
            acc += n_squared;
        }
    }
    println!("1000内🤹之和:{}", acc);

    //用函数式来解决
    let sum_of: u32 = (0..)
        //值转换
        .map(|n| n * n)
        //循环条件，n<1000
        .take_while(|&n| n < upper)
        //过滤元素，🤹
        .filter(|&n| is_odd(n))
        //求和
        .sum();
    println!("Sum of odd numbers: {}", sum_of);
}
