pub fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("vec1中，是否有元素等于2:{:?}", vec1.iter().any(|&x| x == 2));
    //这里整个数组会直接被消耗掉，无法再次使用
    println!("vec2中，是否有元素=2:{}", vec2.into_iter().any(|x| x == 2));

    // println!("ve:{:?}",vec2);
    println!("vec1长度:{}", vec1.len());
    println!("vec1第一个元素:{}", vec1[0]);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2在array1中使用:{}", array1.iter().any(|&x| x == 2));
    //这里的数组元素i32实现了Copy，会消耗副本
    //个人觉得不合理，因为这个会和数组的iter()功能重叠，但是性能还不如iter(),iter是不可变引用
    //1.53后，给数组重新实现了into_iter()，选择了折中方案(顾及现有代码)
    //存储了实现Copy特性元素的数组，复制副本
    println!("5在array2中使用:{}", array2.into_iter().any(|x| x == 5));
    println!("{:?}", array2);
}
