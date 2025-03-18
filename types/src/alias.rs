type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

pub fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    println!(
        "纳秒:{}+英寸:{}={}单位",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
