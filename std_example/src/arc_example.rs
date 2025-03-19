use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let arc = Arc::new("苹果的字符串");
    for _ in 0..10 {
        let arc = Arc::clone(&arc);
        thread::spawn(move || {
            println!("{:?}", arc);
        });
    }
   // thread::sleep(Duration::from_secs(1));
}
