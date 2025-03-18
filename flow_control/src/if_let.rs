enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn main() {
    let number = Some(5);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(5) = number {
        println!("if let 匹配到结果:5")
    }

    //通过if let进行匹配分支并解构
    if let Some(value) = letter {
        println!("letter有值");
    } else {
        println!("letter没有值");
    }

    let i_like_letters = false;

    if let Some(e) = emotion {
        println!("匹配到内容");
    } else if i_like_letters {
        println!("执行不到的分支");
    } else {
        println!("默认分支");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is foobar");
    }

    if let Foo::Qux(x) = c {
        println!("qux matched,c = {}", x);
    }

    //进行匹配并且绑定
    if let Foo::Qux(x @ 100) = c {
        println!("==100,c = {}", x);
    }


    let a=Foo::Bar;


    // 变量 a 匹配 Foo::Bar
    if let Foo::Bar = a {
        // ^-- 这会导致编译时错误。请改用 `if let`。
        println!("a 是 foobar");
    }
}
