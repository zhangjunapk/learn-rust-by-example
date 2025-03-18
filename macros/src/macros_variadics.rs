macro_rules! calculate {
    (eval $e:expr) => {
        println!("{}={}", stringify!($e), $e);
    };
    //定义重载函数
    //函数能接收多个eval
    (eval $e:expr,$(eval $es:expr),+) => {
        calculate!(eval $e);
        //多个eval被递归处理
        calculate!($(eval $es),+);
    };
}

pub fn main() {
    calculate!(
        eval 1+1
    );
    calculate!(
        eval 1+9,
        eval 1+15,
        eval 1+21
    );
}
