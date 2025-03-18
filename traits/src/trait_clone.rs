#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

pub fn main() {
    let unit = Unit;
    let copied_unit = unit;
    println!("原始:{:?}", unit);
    println!("复制后:{:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("原始pair:{:?}", pair);

    let moved_pair = pair;
    println!("移动后的pair:{:?}", moved_pair);

    //pair没有实现Copy，上面就直接移动了
    // println!("原始pair:{:?}",pair);
    let clone_pair = moved_pair.clone();
    drop(moved_pair);
    println!("clone_pair:{:?}", clone_pair);
}
