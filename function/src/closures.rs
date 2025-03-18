pub fn main() {
    let outer_var = 42;
    //闭包是匿名的
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated:{}", closure_annotated(2));
    println!("closure_inferred:{}", closure_inferred(2));

    let one = || 1;
    println!("返回1的闭包: {}", one());


}
