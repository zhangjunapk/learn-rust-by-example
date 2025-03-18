enum Meat {
    Beaf,
    Chiken,
    Fish,
}
struct Washed(Meat);
struct Chopped(Washed);
struct Cooked(Chopped);

fn wash(meat: Option<Meat>) -> Option<Washed> {
    match meat {
        Some(meat) => Some(Washed(meat)),
        None => None,
    }
}

fn chop(washed: Option<Washed>) -> Option<Chopped> {
    match washed {
        None => None,
        Some(washed) => Some(Chopped(washed)),
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    match chopped {
        None => None,
        Some(chopped) => Some(Cooked(chopped)),
    }
}

fn eat(eatable_food: Option<Cooked>) {
    if let Some(Cooked(_)) = eatable_food {
        println!("直接吃，我吃的时候别看我")
    } else {
        println!("吃什么呢????");
    }
}

//这里适合分阶段处理数据的情况
fn process(meat: Option<Meat>) -> Option<Cooked> {
    meat.map(|meat: Meat| Washed(meat))
        .map(|washed: Washed| Chopped(washed))
        .map(|chopped: Chopped| Cooked(chopped))
}

pub fn main() {
    let food = Some(Meat::Beaf);
    let eatable_food = cook(chop(wash(food)));
    eat(eatable_food);

    let food = Some(Meat::Fish);
    let eatable_food = process(food);
    eat(eatable_food);

    let none = process(None);
    eat(none);
}
