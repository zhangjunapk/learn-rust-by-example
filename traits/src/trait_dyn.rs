struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "咩咩咩!!!!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "哞哞哞~~~"
    }
}

//这里的关键字dyn，表示这里是返回一个堆里面存储的固定大小的引用
//引用在堆中是固定的，就知道该分配多少内存了
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if (random_number > 0.5) {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub fn main() {
    let random_number = 0.3;
    let animal = random_animal(random_number);
    println!("获得动物返回,他说:{}", animal.noise());
}
