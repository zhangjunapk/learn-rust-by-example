pub fn main() {
    mine();
    draw_heart();
    mine_eq();
    iter();
    iter_mut();
}

fn mine() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn mine_eq() {
    for n in 1..=100 {
        println!("n={}", n);
    }
}

fn draw_heart() {
    let width = 40;
    let height = 20;
    let mut grid = vec![vec![' '; width]; height];
    for t in (0..628).step_by(2) {
        let t = t as f64 / 100.0;
        let x = 16.0 * t.sin().powi(3);
        let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();

        let x_grid = (x + 20.0) as usize;
        let y_grid = (15.0 - y / 2.0) as usize;

        if x_grid < width && y_grid < height {
            grid[y_grid][x_grid] = '*';
        }
    }
    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!("");
    }
}

//有问题
fn draw_heart_2() {
    let width = 40; // 网格宽度
    let height = 30; // 网格高度
    let mut grid = vec![vec![' '; width]; height];

    // 遍历网格
    for y in 0..height {
        for x in 0..width {
            // 将网格坐标映射到数学坐标
            let x = (x as f64 - width as f64 / 2.0) / 10.0; // x 范围 [-2, 2]
            let y = (height as f64 / 2.0 - y as f64) / 10.0; // y 范围 [-1.5, 1.5]

            // 爱心隐式方程
            let eq = (x * x + y * y - 1.0).powi(3) - x * x * y.powi(3);

            // 如果点接近边界（容差 0.02），标记为爱心
            if eq.abs() < 0.02 {
                grid[y as usize][x as usize] = '*';
            }
        }
    }

    // 打印网格
    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}
//有问题
fn draw_heart_3() {
    let width = 60;
    let height = 40;
    let mut grid = vec![vec![' '; width]; height];

    for y in 0..height {
        for x in 0..width {
            let x = (x as f64 - width as f64 / 2.0) / 15.0;
            let y = (height as f64 / 2.0 - y as f64) / 15.0;
            let eq = (x * x + y * y - 1.0).powi(3) - x * x * y.powi(3);
            if eq.abs() < 0.03 {
                grid[y as usize][x as usize] = '*';
            } else if eq < 0.0 {
                grid[y as usize][x as usize] = '#';
            }
        }
    }

    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn iter() {
    let names = vec!["apple", "orange", "banana"];

    for name in names.iter() {
        match name {
            &"apple" => {
                println!("I'm apple! take it!!!");
            }
            _ => {
                println!("i dont care about it")
            }
        }
    }
    println!("names:{:?}", names);
}
fn into_iter() {
    let persons = vec!["Joseph", "Dan", "Nancy"];
    //这里会直接消耗掉集合，
    for person in persons.into_iter() {
        match person {
            "Joseph" => println!("Joseph"),
            _ => println!("nothing"),
        }
    }
    //再次使用就会提示已经move
    // println!("names:{:?}",persons);
}

fn iter_mut() {
    let mut lang = vec!["java", "rust", "react", "kotlin"];
    //借用集合的每个元素，并且修改值
    for lang in lang.iter_mut() {
        if let "rust"=*lang{
            *lang="rust [love]";
        }

        // *lang = match lang {
        //     &mut "rust" => "Rust [love]",
        //     _ => *lang,
        // }
    }
    println!("lang:{:?}", lang);
}
