use std::marker::PhantomData;
//这里只是用于告诉编译器
struct PhantomTuple<A, B>(A, PhantomData<B>);

struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

pub fn main() {
    let a: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let b: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let c: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let d: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    //类型不匹配
    // println!("{}",a==b);
    // println!("{}",c==d);
}
