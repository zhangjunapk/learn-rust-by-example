fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

fn combine_vec(u: Vec<i32>, v: Vec<i32>) -> impl Iterator<Item = i32> {
    //这个会把另一个vec的迭代器添加到u这个迭代器的后面，链式连接
    u.into_iter().chain(v.into_iter()).cycle()
}

pub fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let mut c = combine_vec(a, b);
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("下一个:{:?}", &c.next());
    println!("全部完成");

    let function = make_adder_function(7);
    //调用闭包
    let a = function(3);
    let b = function(5);
    println!("调用闭包，获取到结果:{}", a);
    println!("调用闭包，获取到结果:{}", b);
    let x = vec![-1, -2, 2, 3];
    let y = double_positives(&x);
    for x in y {
        println!("{}", x);
    }
    for i in x {
        println!("{}", i);
    }
}

//通过函数创建闭包
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    //这里move表示将y的所有权转移到闭包
    let closure = move |x: i32| x + y;
    //返回闭包
    closure
}

//这里显式指定生命周期为'a，指定返回的迭代器生命周期和入参生命周期一致
fn double_positives(nums: &Vec<i32>) -> impl Iterator<Item = i32> {
    nums
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}
