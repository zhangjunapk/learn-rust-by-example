use std::thread;

const THREAD_NUMS: u8 = 10;

pub fn main() {
    let mut threads = Vec::new();
    for i in 0..THREAD_NUMS {
        //这里将i的所有权转移到闭包内部
        //这里u8实现copy，实际上复制了一份
        threads.push(thread::spawn(move || {
            println!("这里是线程: {}", i);
            return i;
        }));
    }
    for thread in threads {
        //这里直接等待线程返回值
        let a = thread.join();
        println!("{:?}", a);
    }
}
