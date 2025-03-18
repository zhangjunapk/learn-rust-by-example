use std::fmt;
use std::fmt::Formatter;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (index, v) in vec.iter().enumerate() {
            write!(f, "{}:{}", index, v)?;
            if (index != vec.len() - 1) {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}

pub fn main() {
    let v = List(vec![7, 8, 9]);
    println!("{}", v);
}
