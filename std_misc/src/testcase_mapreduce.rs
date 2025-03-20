use std::sync::Arc;
use std::thread;

const THREAD_NUM: u8 = 5;
const CHAR_COUNT_PEER_THREAD: usize = 50;

const DATA: &str = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

pub fn main() {
    origin();
    using_buckets();
}

fn origin() {
    let mut join_handlers = Vec::new();

    let all_sp = DATA.split_whitespace();
    for item in all_sp.into_iter() {
        join_handlers.push(thread::spawn(move || {
            let val: Option<u32> = item
                .chars()
                .map(|x| x.to_digit(10)) //转换为十进制
                .sum();
            if let Some(val) = val {
                println!("这一行计算结果:{},共{}个数字", val, item.chars().count());
                val
            } else {
                0
            }
        }));
    }
    let result = join_handlers
        .into_iter()
        .map(|x| x.join().unwrap())
        .sum::<u32>();
    println!("最终计算结果:{:?}", result);
}

//字符串按照空格分割，并且把字符串按照固定长度分割，
// 尽可能让每个线程都能计算均匀
fn using_buckets() {
    let mut thread_handlers = Vec::new();

    let all_chars: Vec<char> = DATA
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|x| x)
        .collect();

    let arc_chars = Arc::new(all_chars);

    let mut chars_peer_thread = arc_chars.to_vec().len() / CHAR_COUNT_PEER_THREAD;
    if (arc_chars.to_vec().len() % CHAR_COUNT_PEER_THREAD != 0) {
        chars_peer_thread += 1;
    }
    for i in 0..chars_peer_thread {
        let start_index = i * CHAR_COUNT_PEER_THREAD;
        let mut end_index = (i + 1) * CHAR_COUNT_PEER_THREAD;
        if (end_index) > arc_chars.to_vec().len() {
            end_index = arc_chars.to_vec().len();
        }
        println!("开始索引:{},结束索引:{}", start_index, end_index);
        let arc_chars = Arc::clone(&arc_chars);
        thread_handlers.push(thread::spawn(move || {
            arc_chars.to_vec()[start_index..end_index]
                .iter()
                .map(|c| {
                    if let Some(val) = c.to_digit(10) {
                        val
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        }));
    }
    let total_value: u32 = thread_handlers
        .into_iter()
        .map(|x| x.join().unwrap())
        .sum::<u32>();
    println!("获取到总值:{}", total_value);
}
