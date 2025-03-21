// cargo test --doc
//这样就能进行文档测试
/// ```
///use unit_test::doc_testing;
/// let result=doc_testing::add(1,2);
///assert_eq!(result,3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/**
```
use unit_test::doc_testing;
fn try_main()->Result<(),String>{
let res=doc_testing::try_div(10,2)?;
Ok(())
}
fn main(){
try_main().unwrap();
}
```
*/
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero error"))
    } else {
        Ok(a / b)
    }
}

