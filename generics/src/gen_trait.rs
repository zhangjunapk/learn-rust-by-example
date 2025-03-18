struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

//为任何调用者来实现接收任意参数
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, t: T) {}
}

pub fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    //所有权转移，无法再次使用
    // empty.double_drop(Null);
}
