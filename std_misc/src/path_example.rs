use std::path::Path;

pub fn main() {
    let path = Path::new(".");
    println!("显示路径:{}", path.display());

    let mut new_path = path.join("a").join("b");

    println!("能否直接修改内部值:{}", path.display());

    println!("join后的路径打印:{}", new_path.display());

    new_path.push("c");
    new_path.push("newfile.tar.gz");
    println!("新文件push后:{}", new_path.display());

    //这里会直接把最后一截替换成下面这个文件名
    new_path.set_file_name("real_file_name.tgz");
    println!("最终的文件名:{}", new_path.display());

    //内部有个os str，最里面保存了u8数组，会直接转换为utf8编码
    match new_path.to_str() {
        Some(os_str) => println!("获得到了str:{}", os_str),
        None => println!("没能获取到str"),
    }
}
