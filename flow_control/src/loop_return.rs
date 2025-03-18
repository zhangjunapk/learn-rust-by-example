pub fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter + 2;
        }
    };
    println!("获得loop结果:{}", result);
}
