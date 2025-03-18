fn drink(beverage: &str) {
    if beverage == "柠檬水" {
        if (cfg!(panic = "abort")) {
            println!("直接终止");
        } else {
            println!("快，催吐");
        }
    } else {
        println!("享受饮品的时间:{}", beverage);
    }
}

//panic 级别默认是unwind
//遇到错误时，会展开堆栈信息，方便调试
#[cfg(panic = "unwind")]
fn ah() {
    println!("unwind panic级别");
}

//abort不会展开堆栈信息，只是终止程序，清理资源

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("不是unwind panic 级别")
}

pub fn main() {
    let s = "柠檬水";
    if (s == "柠檬水") {
        ah();
    } else {
        println!("真好喝:{}", s);
    }

    drink("water");
    drink("柠檬水");
}
