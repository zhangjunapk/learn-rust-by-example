use function as other_function;

fn function() {
    println!("调用了function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("deeply::nested::function()");
        }
    }
}

pub fn main() {
    other_function();
    println!("进入代码块");
    {
        use crate::mod_use::deeply::nested::function;
        function();
        println!("代码块尾部")
    }
    function();
}
