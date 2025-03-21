pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
pub fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.9;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("负浮点数没有平方根".to_owned())
    }
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("初0异常");
    } else if (a < b) {
        panic!("结果为0")
    }
    a / b
}

#[cfg(test)]
mod tests_two{
    use super::*;
    #[test]
    fn test_divide(){
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }
    #[test]
    #[should_panic]
    fn test_any_panic(){
        divide_non_zero_result(10,0);
    }

    #[test]
    #[ignore]
    fn aaa(){
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }


}