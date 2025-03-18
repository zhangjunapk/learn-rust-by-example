#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}
pub fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);

    let default_none = Option::<Fruit>::None;
    //这里有检测，如果自己有值是Some,会直接返回自己，不然就返回传递到or函数中的参数，
    //这样就被消耗掉了
    //这里的orange虽然在内部执行时返回的self，但是方法调用了，orange作为参数传入了or
    //那么这个orange变量的所有权也被转移了，因此这里也不能再使用了，所有权也被转移了
    let fruits = default_none.or(apple).or(orange);
    if let Some(fruit) = fruits {
        println!("选中的水果:{:?}", fruit);
    }
    //这里被上面的or 消耗掉了，转移了所有权
    // println!("再次打印试试:{:?}",apple)
    //这里的所有权也被转移了，因为调用了or函数，传递了移交所有权
    // println!("试试打印橘子:{:?}", orange);

    let none_option = Option::<Fruit>::None;
    //这里的闭包没有捕获外部变量，因此他是自动实现了Copy特性
    //这里的闭包是Fn，因为没有捕获外部变量
    let get_kiwi_as_fallback = || {
        println!("调用打印方法，猕猴桃作为备选");
        Some(Fruit::Kiwi)
    };

    //这里隐式声明，这里是个Fn，能够被多次调用
    let get_banana_as_fallback = || {
        println!("香蕉作为备选水果");
        Some(Fruit::Banana)
    };

    //fn 继承自FnMut
    //FnMut 继承自FnOnce
    //因此Fn能够作为参数传递到or_else
    let get_fruit_using_or_else = none_option
        //这里的or_else接收移动所有权并且有FnOnce特性的函数或闭包，会移动所有权
        //但是上面的闭包没有捕获外部变量，因此被隐式生命为Fn，并且自动实现了Copy特性
        //这里的复制是指，复制闭包的指针
        .or_else(get_kiwi_as_fallback)
        .or_else(get_banana_as_fallback);
    println!("试试or_else:{:?}", get_fruit_using_or_else);
    let got_fruit_using_function = get_banana_as_fallback().unwrap();

    println!("直接通过函数来获取水果:{:?}", got_fruit_using_function);

    let x = String::from("aaaaa");
    let closure = || println!("变量值:{}", x);
    closure();
    //这里会复制闭包的指针，来传递到另一个函数内
    t(closure);
    closure();

    let mut none_fruit = Option::<Fruit>::None;
    let a = Fruit::Apple;
    //这里会对参数进行所有权转移
    let got = none_fruit.get_or_insert(a);
    println!("get_or_insert:{:?}", got);
    println!("get_or_insert:{:?}", none_fruit);

    //所有权被转移了，无法再次打印
    // println!("测试苹果是否还能够打印:{:?}", a);

    let mut none_fruit = Option::<Fruit>::None;
    let closure_lemon = || Fruit::Lemon;

    //如果是None，会直接修改当前值
    none_fruit.get_or_insert_with(closure_lemon);

    let closure_banana = || Fruit::Banana;

    none_fruit.get_or_insert_with(closure_banana);
    println!("水果打印:{:?}", none_fruit);
}

fn t<F>(f: F)
where
    F: FnOnce(),
{
    f()
}
