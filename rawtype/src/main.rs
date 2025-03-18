use rawtype::literals;
use rawtype::tuples;
use rawtype::array;

pub fn main() {
    mine_main();
    literals::main();
    tuples::main();
    array::main();
}
fn mine_main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 51151551;

    let mut mutable = 12;
    mutable = 36;
    let mutable = true;

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let your_single = (6);
    let my_tuple = (5u32, 1u8, true, -5.251f32);
}
