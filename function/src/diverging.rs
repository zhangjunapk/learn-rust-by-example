fn foo() -> ! {
    panic!("this panic is fine");
}

fn some_fn() {
    ()
}

pub fn main() {
    let _a: () = some_fn();
    println!("能看见这一行");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut result = 0;
        for i in 0..=up_to {
            //写rust需要想象力
            if let add = match i % 2 == 1 {
                true => i,
                //想象力
                false => continue,
            } {
                result += add
            }

            let add: u32 = match i % 2 == 1 {
                true => i,
                //想象力
                false => continue,
            };
            result += add;
        }
        result
    }

    let res = sum_odd_numbers(10);
    println!("小于10的🤹和:{}", res);

    let b = foo();
    println!("try");
}
