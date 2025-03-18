pub fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("x的第一个元素1,b:{},y:{}", b, y),
        Foo { y: 2, x: i } => println!("y为2,i={:?}", i),
        Foo { y, .. } => println!("不关心x的值,y:{}", y),
    }

    let faa = Foo { x: (3, 5), y: 9 };

    let Foo { x: tx, y: ty } = faa;
    println!("通过let解构结构体，x:{:?},y:{:?}", tx, ty);

    struct Bar {
        foo: Foo,
    }
    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo { x: tx, y: ty },
    } = bar;
    println!("解构复杂结构体，x:{:?},y:{:?}", tx, ty);
}
