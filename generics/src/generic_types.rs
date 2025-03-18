struct Container(i32, i32);

trait Contains {
    //这个没必要吧，这个还能通过where来指定
    //还能直接用泛型写法
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = (i32);
    type B = (i32);

    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}
fn difference<C:Contains>(container: &C)->i32{
    container.last() - container.first()
}

pub fn main(){
    let num1=3;
    let num2=10;

    let container=Container(num1, num2);

    println!("容器是否包含{}和{}={}",&num1,&num2,container.contains(&num1,&num2));
    println!("第一个数字:{}",&container.first());
    println!("最后一个数字:{}",&container.last());
    println!("差值:{}",difference(&container));
}