use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

pub fn main() {
    let vec = read_lines_simple("hi.txt");
    vec.into_iter().for_each(|f| println!("{}", f));

    //这里直接使用他内部实现的迭代器进行遍历

    match read_line_using_buf("hi.txt") {
        Ok(lines) => {
            //这里的map_while实现了迭代器，能够遍历
            for line in lines.map_while(Result::ok) {
                println!("读取到行:{}", line);
            }
        }
        Err(e) => {
            println!("读取文件遇到错误:{:?}", e);
        }
    }
}

fn read_lines(file_name: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in fs::read_to_string(file_name).unwrap().lines() {
        result.push(String::from(line));
    }
    result
}
fn read_lines_simple(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn read_line_using_buf<P>(fle_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(fle_name)?;
    Ok(io::BufReader::new(file).lines())
}
