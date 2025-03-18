use scope::*;
fn main() {
    raii::main();
    scope_move::main();
    move_mut::main();
    partial_move::main();
    borrow_alias::main();
    lifetime_explicit::main();
    lifetime_fn::main();
    lifetime_methods::main();
    lifetime_struct::main();
    lifetime_trait::main();
    lifetime_bounds::main();
    lifetime_coercion::main();
    lifetime_static::main();
    lifetime_elision::main();
}
