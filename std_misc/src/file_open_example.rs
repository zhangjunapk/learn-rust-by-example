use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn main() {
    let path = Path::new("hi.txt");
    println!("Opening: {}", path.display());
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        let usize = file.read_to_string(&mut content);
        if let Ok(usize) = usize {
            println!("读取到内容：{},大小:{}", content, usize);
        }
    }
}
