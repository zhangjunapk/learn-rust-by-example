fn check_division(num: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        println!("除数为0,无法计算结果");
        None
    } else {
        Some(num / divisor)
    }
}
fn calculation(num: i32, divisor: i32) {
    match check_division(num, divisor) {
        None => {
            println!("没能计算出结果");
        }
        Some(res) => {
            println!("{}/{}={}", num, divisor, res);
        }
    }
}
pub fn main() {
    calculation(10, 2);
    calculation(10, 0);
    println!("none解包预期会报错:{}", None::<i32>.unwrap());
}
