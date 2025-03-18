#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let c = 'Q';
    //左侧ref等于右侧&
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("{}", *ref_c1 == *ref_c2);

    let point = Point { x: 300, y: 900 };

    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    //Point实现了Copy，直接复制，并可变
    let mut mutable_point = point;
    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_,ref mut last) = mutable_tuple;
        *last=2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
