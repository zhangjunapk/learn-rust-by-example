use std::fmt::{Display, Formatter};

enum Day {
    Monday,
    Saturday,
    Sunday,
}

enum Food {
    Sushi,
    Chicken,
    Beef,
}

impl Display for Food {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Food::Sushi => {
                write!(f, "寿司")
            }
            Food::Chicken => {
                write!(f, "熬的鸡肉汤")
            }
            Food::Beef => {
                write!(f, "5分熟牛排🐮")
            }
        }
    }
}

fn order_able(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        Food::Chicken => Some(food),
        Food::Beef => Some(food),
    }
}

fn has_material(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        Food::Chicken => None,
        Food::Beef => Some(food),
    }
}

fn cookie(food: Food) -> Option<Food> {
    //这里调用函数has_material
    //然后调用and_then，
    //and_then会解包Option出来原始的数据，再调用函数，得到Option
    has_material(food).and_then(order_able)
}

fn eat(food: Food) {
    match cookie(food) {
        Some(food) => println!("吃上一顿饱饭了啊，开心啊,{}", food),
        None => println!("啊，吃不上啊，那算了。"),
    }
}

pub fn main() {
    eat(Food::Sushi);
    eat(Food::Beef);
    eat(Food::Chicken);
}
