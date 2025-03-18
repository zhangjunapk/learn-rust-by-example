fn main() {
    fizzbuzz_to(100);
    function::methods::main();
    function::capture::main();
    function::closures_input::main();
    function::closures_anonymity::main();
    function::funinput::main();
    function::closure_output::main();
    function::iter_any::main();
    function::iter_find::main();
    function::hof::main();
    function::diverging::main();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 1 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("n={}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
