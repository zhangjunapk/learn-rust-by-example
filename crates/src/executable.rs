//这里不应该报错，
// rustc executable.rs --extern rary=library.rlib && ./executable
extern crate rary;


pub fn main(){
    rary::public_function();

}