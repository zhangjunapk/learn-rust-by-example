//会根据cfg来编译进去函数
//这个函数会被编译进去
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

//这个函数不会被编译进去
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

pub fn main() {
    are_you_on_linux();
    println!("Are you sure?");
    if (cfg!(target_os = "linux")) {
        println!("You are running linux!");
    } else {
        println!("You are *not* running linux!");
    }
}
