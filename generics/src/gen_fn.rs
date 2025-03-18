struct A;
struct S(A);
//定义泛型结构体
struct SGen<T>(T);

fn reg_fn(s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

//这个才是泛型函数
fn generic<T>(_s: SGen<T>) {}

pub fn main() {
    reg_fn(S(A));

    gen_spec_t(SGen(A));

    gen_spec_i32(SGen(6));

    //显式指定函数泛型
    generic::<String>(SGen(String::from("aaaaa")));
    //隐式使用
    generic(SGen(6));
}
