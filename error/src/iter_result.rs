pub fn main() {
    filter_map();
    map_err();
    collect();
    partition();
}

fn map() {
    let strings = vec!["tofu", "33"];
    //这里会把原始数组转换为i32的Result放到新的数组中
    let nums: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("{:?}", nums);
}
fn filter_map() {
    let strings = vec!["tofu", "33"];
    let vec = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    println!("{:?}", vec);
}

fn map_err() {
    let strings = vec!["tofu", "33"];
    let mut errs = vec![];
    let res = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        //这里可以对处理结果再次过滤下
        //如果匹配到错误，添加到一个保存错误的数组中
        .filter_map(|r| r.map_err(|e| errs.push(e)).ok())
        .collect::<Vec<i32>>();
    println!("{:?}", res);
    println!("{:?}", errs);
}

fn collect() {
    let strings = vec!["tofu", "33"];
    //普通的map+collect，遇到错误会直接返回
    let res: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("collect:{:?}", res);
}

fn partition() {
    let strings = vec!["tofu", "33"];
    let (nums, errs): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        //这里会根据传递的闭包进行分别存放
        .partition(|r| r.is_ok());
    println!("partition分区得到的错误:{:?}", errs);
    println!("partition获取到的结果:{:?}", nums);
}
