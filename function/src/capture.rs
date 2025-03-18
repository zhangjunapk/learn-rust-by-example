use std::mem;

pub fn main() {
    let color = String::from("green");
    let print = || println!("{}", color);
    print();

    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;

    //这个闭包在内部需要修改捕获的变量，因此需要被mut修饰
    //表示需要捕获count变量为可变引用
    let mut inc = || {
        count += 1;
        println!("{}", count);
    };
    inc();

    // let _reborrow=&count;
    inc();

    //闭包执行完毕，可以再次借用
    let _coreborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable is :{}", movable);
        mem::drop(movable);
    };
    //这里只能被调用一次，因为被销毁了
    consume();

    let haystack = vec![1, 2, 3];

    //vec所有权被转移到闭包内部
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("{:?}", haystack); 报错，因为被上面的闭包借走了
}
