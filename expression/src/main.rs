fn main() {
    let x = 5;
    x;
    x + 1;
    15;

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x; //这个分号会抑制赋值给z，这样()会赋值给z
    };

    println!("x是:{}", x);
    println!("y是:{}", y);
    println!("z是:{:?}", z);
}
