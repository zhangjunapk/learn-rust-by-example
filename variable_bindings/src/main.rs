use variable_bindings::mut_variable;
use variable_bindings::scope;
use variable_bindings::shadow;
use variable_bindings::declare;
use variable_bindings::freeze;

fn main() {
    mine();
    mut_variable::main();
    scope::main();
    shadow::main();
    declare::main();
    freeze::main();
}

fn mine() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("整数:{}", copied_integer);
    println!("布尔值:{}", a_boolean);
    println!("单元值:{:?}", unit);

    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}
