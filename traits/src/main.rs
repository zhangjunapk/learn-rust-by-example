use traits::*;

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{}说:{}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if (self.is_naked()) {
            println!("已经剃过毛了")
        } else {
            println!("开始给🐏剃毛");
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep { naked: false, name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if (self.naked) {
            "咩咩咩???"
        } else {
            "咩咩咩~"
        }
    }
    fn talk(&self) {
        println!("i am sheep ,i am talking {}", self.noise());
    }
}

fn main() {
    let mut mn: Sheep = Animal::new("穆妮");
    &mn.talk();
    &mn.shear();
    &mn.talk();
    trait_dyn::main();
    trait_ops::main();
    trait_drop::main();
    trait_iter::main();
    trait_impl::main();
    trait_clone::main();
    trait_supertraits::main();
    trait_disambiguating::main();
}
