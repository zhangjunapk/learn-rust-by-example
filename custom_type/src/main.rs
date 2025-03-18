use custom_type::structs;
use custom_type::enums;
use custom_type::enum_use;
use custom_type::eum_c_like;
use custom_type::enum_list;
use custom_type::constants;
fn main(){
    structs::main();
    enums::main();
    enum_use::main();
    eum_c_like::main();
    enum_list::main();
    constants::main();
}