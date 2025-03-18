use generics::*;

fn foo<T>(arg: T) {}

struct A;
struct Single(A);

struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    let a: SingleGen<char> = SingleGen('a');

    let b: SingleGen<i32> = SingleGen(3i32);
    let c: SingleGen<String> = SingleGen(String::from("a"));

    generic_impl::main();
    gen_trait::main();
    // a::main();
    bounds::main();
    generic_where::main();

    new_types::main();
    generic_problem::main();
    generic_types::main();
    generic_units::main();
}
