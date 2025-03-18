fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1
}

fn print_multi<'a, 'b>(x: &'a i32, b: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, b);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

//这里超过函数的作用域了，超出后，String会被清理
//因此返回值的生命周期不能和函数一致
/*fn invalid_output<'a>()->&'a String{
    &String::from("not implemented")
}*/

pub fn main() {
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    //这里可变引用传递进去，会作为不可变引用来使用
    print_one(&t);
}
