//rustc --crate-type=lib rary.rs
//生成外部能引入的库
pub fn public_function(){
    println!("调用了rary的public function");
}
fn private_function(){
    println!("调用了private function");
}
pub fn indirect_access(){
    println!("调用了indirect_access");
    private_function();
}