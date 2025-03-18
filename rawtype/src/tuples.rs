use std::fmt;
use std::fmt::Formatter;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "({} {})", self.0, self.1)?;
        writeln!(f, "({} {})", self.2, self.3)
    }
}

pub fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("元组第一个值:{}", long_tuple.0);
    println!("元组第二个值:{}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16), ('a', true));
    println!("元组的元组:{:?}", tuple_of_tuples);

    //最长能打印输出含有12个元素的元组
    let tuple_too_long = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("过长的元组，测试能否打印:{:?}", tuple_too_long);

    let pair = (1, true);

    println!("反转元组:{:?}", reverse(pair));

    println!("单元素对象:{:?}", (1i8,));
    println!("一个整数:{:?}", (1i8));

    let tuple = (1, true, 'a', 0.36f64);
    let (a, b, c, d) = tuple;
    println!("{:?},{:?},{:?}.{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("原矩阵:");
    println!("{}", matrix);
    println!("转置:");
    let t = transpose(matrix);
    println!("{}",t);
}
