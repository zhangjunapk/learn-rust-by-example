pub fn main() {
    //通过迭代器来获取数字并且转换为vec
    let mut nums: Vec<u32> = (0..10).collect();
    //得到的数组还能够再次添加值
    nums.push(20);
    println!("数组输出:{:?}", nums);

    let mut nums = vec![1, 2, 3];
    println!("弹出左右一个值:{}", nums.pop().unwrap());
    nums.push(4);
    println!("nums:{:?}", nums);
    println!("指定索引的数据:{}", nums[1]);
    //索引找不到数据直接引发panic
    // println!("指定索引的数据:{}", nums[500]);

    for x in nums.iter() {
        println!("数组内元素值:{}", x);
    }
    //这里能够实现带索引的遍历
    for (i, x) in nums.iter().enumerate() {
        println!("索引:{},值:{}", i, x);
    }
    //通过iter_mut能够已可变引用的方式来遍历数据，并且修改数据
    for x in nums.iter_mut() {
        *x += 1
    }
    println!("nums:{:?}", nums);
}
