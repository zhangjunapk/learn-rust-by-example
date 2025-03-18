fn function() {
    println!("调用function")
}
mod cool {
    pub fn function() {
        println!("调用了cool::function")
    }
}
mod my {
    fn function() {
        println!("调用了my::function")
    }
    mod cool {
        pub fn function() {
            println!("调用my::cool::function")
        }
    }
    pub fn indirect_call() {
        println!("调用了my::indirect_call");
        self::function();
        function();

        self::cool::function();
        //在这个rs文件里面定义的函数，会被当作在这个rs文件里面定义的第一层结构体的父
        super::function();

        //直接从crate开始调用
        crate::mod_super::function();

        {
            use crate::mod_super::cool::function as root_function;
            root_function();
        }
    }
}

pub fn main() {
    my::indirect_call();
}
