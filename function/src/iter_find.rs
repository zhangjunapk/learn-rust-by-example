pub fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    //内部都是接收FnMut(&Self.Item) 来表示每个元素
    //iter是不可变引用传递，于是这里是&(&i32)
    println!("在vec1中查找2:{:?}", iter.find(|&&x| x == 2));
    //into_iter()是所有权转移，这个item就直接是i23
    println!("在vec2中查找2:{:?}", into_iter.find(|&x| x == 2));
    // println!("尝试打印vec2:{:?}",vec2) 被移动

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("在array1中查找2:{:?}", array1.iter().find(|&&x| x == 2));

    println!("在array2中查找2:{:?}", array2.into_iter().find(|&x| x == 2));

    let vec=vec![1,2,3,4,5];
    let find4_=vec.iter().position(|&x| x == 4);
    if let Some(index)=find4_{
        println!("found4 position={}", index);
    }
    //这个position，第一次匹配就直接返回一个index
    let find_=vec.into_iter().position(|x| x % 2 == 0);
    if let Some(index)=find_{
        println!("found2 position={}", index);
    }
}
