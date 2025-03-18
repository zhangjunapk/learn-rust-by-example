pub fn main() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("short lived binding:{}", short_lived_binding);
    }
    println!("long lived binding:{}", long_lived_binding);
    //作用域不同
    //println!("short lived binding :{}",short_lived_binding);
}
