use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a:expr,$b:expr,$func:ident) => {
        //检测a和b表达式的长度是否匹配
        assert!(
            $a.len() == $b.len(),
            "{:?}:维度不匹配:{:?} {:?}",
            stringify!($func),
            ($a.len(),),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident,$bound:ident,$method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func);
            //遍历x和y可变长度数组，并且对每个x元素重新赋值
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                //$bound 表示使用std::ops::下面的哪个特性
                //$method 表示使用特性下面的哪个方法
                *x = $bound::$method(*x, *y);
            }
        }
    };
}
//通过宏来定义函数，这个函数的作用是对两个可变数组进行遍历，并且修改a数组的每个元素
op!(add_assign, Add, add);
op!(mul_assign, Mul, mul);
op!(sub_assign, Sub, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident,$x:expr,$y:expr,$z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
                println!("")
            }
        };
    }
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
