#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NameBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NameBorrowed { x: &y, y: &x };
    let reference = Either::Ref(&x);
    let number = Either::Num(x);

    println!("x在结构体中被借用:{:?}",single);
    println!("x和y在结构体中被借用:{:?}",double);
    println!("x被借用:{:?}",reference);
    println!("x被借用:{:?}",number);
}
