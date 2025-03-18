#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("不可变借用,<{}> 版:{}.", book.title, book.year);
}
fn new_edition(book: &mut Book) {
    book.year = 2024;
    println!("书的版本更新了")
}

pub fn main() {
    let immutabook = Book {
        author: "荣格",
        title: "追梦，回忆与思考",
        year: 2023,
    };
    //如果没有copy，这里会进行所有权转移
    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    //不可变引用，不能借用
    // new_edition(&mut immutabook);
}
