pub fn main() {
    'outer:loop{
        println!("外层循环");
        'inner:loop{
            println!("内层循环");
            break 'outer;
        }
        println!("外层循环永远不会到达");
    }
    println!("推出外层循环");
}
