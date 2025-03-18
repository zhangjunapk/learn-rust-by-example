mod my_mod {
    fn private_function() {
        println!("调用了私有函数");
    }
    pub fn function() {
        println!("调用公共函数");
    }
    pub fn indirect_access() {
        println!("开始调用私有函数");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("调用nested公共函数");
            private_function();
        }
        fn private_function() {
            println!("调用nested:私有函数")
        }
        //相当于my_mod中的私有函数
        pub(in crate::visibility::my_mod) fn pub_function_in_mod() {
            println!("调用了在my_mod中可见的函数");
            pub_self_nested();
        }

        //相当于私有函数(这样定义容易带来混淆，可读性不强)
        pub(self) fn pub_self_nested() {
            println!("只能在当前模块可见")
        }

        //相当于在他爹的模块中的私有函数
        pub(super) fn pub_super_function() {
            println!("只能在super(my_mod)中调用")
        }
    }
    pub fn call_function_in_mymod() {
        //调用在my_mod可见的函数
        nested::pub_function_in_mod();
        //调用nested模块中，在父类可见的函数
        nested::pub_super_function();
        //调用公共函数
        nested::function();
    }
    pub(crate) fn pub_crate_function() {
        println!("只能在当前crate可见");
    }

    //私有模块
    mod private_nested {
        pub fn function() {
            println!("my_mod下面的私有模块")
        }
        pub(crate) fn restricted_crate_function() {
            println!("调用了my_mod下面私有模块的crate函数")
        }
    }

    fn a() {
        private_nested::function();
    }
}

fn function() {
    println!("私有函数");
}

pub fn main() {
    //当前rs中的私有函数
    function();
    //私有模块的公共函数
    my_mod::function();

    //私有模块的公共函数
    my_mod::indirect_access();
    //私有模块下面的公共模块的公共函数
    my_mod::nested::function();
    //私有模块下面的公共函数
    my_mod::call_function_in_mymod();
    //私有模块下，只允许自己调用的pub函数
    my_mod::pub_crate_function();

    //这个只能在my_mod内部调用
    //比如在my_mod中定义函数，在函数内部调用
    // my_mod::nested::pub_function_in_mod();

    //这个是private函数，只能在my_mod内部调用
    // my_mod::private_function();

    //只能在nested内部调用，私有函数
    // my_mod::nested::private_function();

    //这个是私有模块，只能在定义处同级调用
    // my_mod::private_nested::function();

    //私有模块，只能在同级处调用
    // my_mod::private_nested::restricted_crate_function();
}
