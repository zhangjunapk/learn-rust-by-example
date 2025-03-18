use std::ops::RangeInclusive;

struct Fibonacci {
    curr: u32,
    next: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

pub fn main() {
    let mut sequence: RangeInclusive<i32> = 0..=3;
    println!("连续调用4次next");
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());

    //for 使用into_iter()来将所有权进行转移并遍历
    for i in 0..3 {
        println!("{}", i);
    }

    //因为for是into_iter，所以每次都通过函数重新生成新的对象
    //这里只获取前10个元素
    for i in fibonacci().take(10) {
        println!("{}", i);
    }

    //跳过前4个，获取共十个元素
    for i in fibonacci().skip(4).take(10) {
        println!("{}", i)
    }

    let array = [1u32, 3, 4, 8];

    println!("遍历数组:{:?}", array);
    //这里的iter是不可变引用
    for i in array.iter() {
        println!("{}", i);
    }
    //这里的println调用，传入的也是不可变引用
    println!("遍历数组:{:?}", array);
}
