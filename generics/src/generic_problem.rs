struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, num1: &i32, num2: &i32) -> bool {
        (&self.0 == num1) && (&self.1 == num2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    //借用
    fn last(&self) -> i32 {
        self.1
    }
}
pub fn main() {
    let container = Container(1, 2);
    println!("第一个值：{}", container.first());
    println!("第一个值：{}", container.last());
    println!("第一个值：{}", container.first());
    container.contains(&5, &9);
    //i32实现了Copy，作为方法返回值会复制
    container.last();
    container.first();
    container.last();
}
