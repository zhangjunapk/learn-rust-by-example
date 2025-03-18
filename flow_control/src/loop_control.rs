pub fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if (count == 3) {
            continue;
        }
        println!("It's {} ", count);
        if (count == 5) {
            break;
        }
    }
}
