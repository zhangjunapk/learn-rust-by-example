use std::sync::mpsc;
use std::thread;

static THREADS: u8 = 5;

pub fn main() {
    let mut join_handlers = Vec::new();
    let (sender, receiver) = mpsc::channel::<u8>();
    for id in 0..THREADS {
        let sender = sender.clone();
        join_handlers.push(thread::spawn(move || {
            sender.send(id).unwrap();
        }))
    }

    //这里的mpsc是多生产者，单消费者
    //multiple producer single consumer
    loop {
        let message = receiver.recv().unwrap();
        println!("接收到消息:{}", message);
    }
}
