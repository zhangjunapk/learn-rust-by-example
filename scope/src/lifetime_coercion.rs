//自动推断生命周期，会自动推断生命周期一致
fn multiple<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

//显式指定生命周期，a强制和b一致
fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    first
}

pub fn main(){
    let first=2;
    {
        let second=3;
        println!("乘积是:{}",multiple(&first, &second));
        println!("{}是第一个",choose_first(&first, &second));
    }
}