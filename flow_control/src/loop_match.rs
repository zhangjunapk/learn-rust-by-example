pub fn main() {
    let mut optional = Some(0);
    loop {
        match optional {
            None => {
                break;
            }
            Some(value) => {
                if (value > 9) {
                    break;
                } else {
                    optional = Some(value + 1);
                    println!("value自增:{}", value + 1);
                }
            }
        }
    }

    //解构+循环
    //循环条件是optional能够解构成Some
    while let Some(i) = optional {
        if i >= 19 {
            break;
        } else {
            optional = Some(i + 1);
            println!("value:{}", i + 1);
        }
    }

    optional = Some(10 + 1);
}
