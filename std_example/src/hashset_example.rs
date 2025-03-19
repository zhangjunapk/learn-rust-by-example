use std::collections::HashSet;

pub fn main() {
    let mut a: HashSet<i32> = vec![1, 2, 1, 2, 2].into_iter().collect();
    let mut b: HashSet<i32> = vec![2, 5, 3, 3, 5].into_iter().collect();
    println!("{:?}", a);
    println!("{:?}", b);

    a.insert(4);
    println!("是否包含4:{}", a.contains(&4));

    println!("a集合内容:{:?}", a);
    println!("b集合内容:{:?}", b);

    println!("并集:{:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("差集:{:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!("交集:{:?}",a.intersection(&b).collect::<Vec<&i32>>());
    //这个对称集是指，元素在一个集合中有，在另一个集合中没有
    println!("对称集:{:?}",a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
