pub fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("{}fizzbuzz", n);
        } else if n % 3 == 0 {
            println!("{}fizz", n);
        } else {
            println!("{}buzz", n);
        }
        n += 1;
    }
}

