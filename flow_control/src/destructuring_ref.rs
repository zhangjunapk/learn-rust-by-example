pub fn main() {
    let reference = &4;
    match reference {
        &val => println!("got you ,my val:{:?}", val),
    }
    match *reference {
        val => println!("这里直接是解引用后的值:{:?}", val),
    }

    let _not_a_ref = 3;

    let ref _if_a_ref = 3;

    let value = 5;

    //这里直接创建一个副本(因为i32实现了Copy特性)
    match value {
        //单分支，会直接执行(匹配任何值)
        //这里直接创建一个执行副本的引用
        ref val => {
            println!("val:{:?}", val)
        }
    }

    let mut mut_value = 6;
    match mut_value {
        //这里因为字段被mut修饰，再加上下面的匹配加了关键字ref mut ,
        // 就没有复制一份了，直接用&mut引用，尽管i32实现了Copy特性
        ref mut m => {
            *m += 10;
            println!("累加后:{}", m);
            println!("累加后:{}", mut_value);
        }
    }
    println!("累加后:{}", mut_value);
}
