fn give_adult(drink: Option<&str>) {
    match drink {
        None => {
            println!("没有饮料呢，大胆???!!!")
        }
        Some("柠檬水") => {
            println!("酸酸的，真解渴啊")
        }
        Some(inner) => {
            println!("好喝呢:{}", inner);
        }
    }
}

fn drink(drink: Option<&str>) {
    if let Some("柠檬水") = drink {
        panic!("shit~ 酸死爸爸了,直接死给你看~!!!???")
    }
    //unwrap None会直接报错
    println!("喝点:<{}>，又不是不能喝", drink.unwrap());
}

pub fn main() {
    let lemon_juice = Option::<&str>::Some("柠檬水");
    give_adult(lemon_juice);

    let water = Option::<&str>::Some("长白山纯净水");
    give_adult(water);

    give_adult(Option::<&str>::None);

    drink(Option::<&str>::Some("童子尿"));

    drink(Option::<&str>::Some("柠檬水"));

    //这里传递None，在内部unwrap会直接报错
    drink(Option::<&str>::None);
}
