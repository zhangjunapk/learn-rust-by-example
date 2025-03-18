use crate::enum_list::List::{Cons, Nil};

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => {
                1 + tail.len()},
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}
pub fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("链表长度为:{}", list.len());
    println!("{}", list.stringify());
}
