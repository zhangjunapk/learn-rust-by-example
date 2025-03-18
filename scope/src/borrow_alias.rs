struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "点坐标:{},{},{}",
        borrowed_point.x, another_borrow.y, point.z
    );
    //不可变引用正在借用期间，不能再次被借用为可变
    // let mutable_borrow = &mut point;
    println!(
        "点的坐标:{},{},{}",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    //可变借用的scope还没结束，不能再次被可变借用
    // let y = &point.y;
    //
    // println!("{}", point.z);

    println!(
        "点坐标:{},{},{}",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    //可变借用scope结束，可以再次被借用

    let new_borrowed_point = &point;
    //可变引用作为不可变引用传递进去来打印
    println!(
        "{},{},{}",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
