pub fn main() {
    let array = [1, -2, 6];

    match array {
        //通过match来结构数组，第一个元素是0的情况，第二第三个元素绑定
        [0, second, third] => {
            println!("第一个元素是0的情况");
        }
        /*[1, _, third] => {
            println!("第一个元素是1,第二个元素被忽略");
        }*/
        [-1, second, ..] => {
            println!("第一个元素是-1,第二个也绑定过来了，其他元素被忽略");
        }
        [3, second, tail @ ..] => {
            println!(
                "第一个元素是3,第二个绑定:{}，其他元素绑定到同一个切片中:{:#?}",
                second, tail
            );
        }
        [first, middle @ .., last] => {
            println!(
                "第一个参数绑定:{}，最后一个参数绑定:{}，中间的都绑定到切片中:{:#?}",
                first, last, middle
            )
        }
    }
}
