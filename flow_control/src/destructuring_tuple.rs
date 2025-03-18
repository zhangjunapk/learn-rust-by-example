pub fn main() {
    let triple = (0, -2, 3);
    println!("Tell me about: {:?}", triple);
    //啊，怎么还有这种写法，牛逼，想法真好
    match triple {
        (0, y, z) => {
            println!("元组第一个元素值是0,{:?}", triple);
        },
        (1, ..) => {
            println!("第一个是1,其他元素不管");
        },
        (.., 2) => {
            println!("最后一个元素是2,其他不管");
        },
        (3, .., 4) => {
            println!("第一个元素是3,中间不管，最后一个元素是4");
        },
        (_, _, _) => {
            println!("no match");
        }
    }
}
