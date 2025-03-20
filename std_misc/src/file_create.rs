use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    let path = Path::new("rust-fedora.txt");

    if let Ok(mut file) = File::create(&path) {
        match file.write_all("文件内容会被清空，如果存在".as_bytes()) {
            Ok(_) => {
                println!("文件写入成功")
            }
            Err(e) => {
                println!("文件写入失败:{:?}", e);
            }
        }
    } else {
        println!("文件创建失败");
    }
}
