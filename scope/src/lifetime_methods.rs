struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn main() {
    let mut owner = Owner(100);

    owner.add_one();
    owner.print();
}
