use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn cat(mut current_path: &PathBuf, file_name: &str) {
    let mut path = current_path.clone();
    path.push(file_name);
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        file.read_to_string(&mut content);
        println!("{}", content);
    } else {
        println!("遇到错误，无法读取文件")
    }
}

//如果已经存在文件，就清空再重新写入
fn echo(path: &PathBuf, params: &[&str]) {
    match params {
        [content] => {
            println!("{}", content)
        }
        [content, ">", file_name] => {
            let mut path = path.clone();
            path.push(file_name);
            if let Ok(mut file) = File::create(path) {
                if let Err(err) = file.write_all(content.as_bytes()) {
                    println!("文件写入失败:{:?}", err);
                }
            } else {
                println!("文件创建失败");
            }
        }
        &[] | &[_, _] | &[_, _, _, _, ..] => todo!(),
        &[_, &_, _] => todo!(),
    }
}

//新建文件，如果存在就不新建
fn touch(current_path: &PathBuf, file_name: &str) -> io::Result<File> {
    let mut current_path = current_path.clone();
    current_path.push(file_name);
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(current_path)
}

fn mkdir(current_path: &PathBuf, params: &[&str]) {
    let mut path = current_path.clone();
    match params {
        ["-p", file_path] => {
            path.push(file_path);
            if let Err(err) = fs::create_dir_all(path) {
                println!("创建文件夹遇到错误:{}", err);
            }
        }
        [file_path] => {
            path.push(file_path);
            if let Err(err) = fs::create_dir(path) {
                println!("创建多级文件夹遇到错误:{}", err)
            }
        }
        &[] | &[_, _, _, ..] => todo!(),
        &[&_, _] => todo!(),
    }
}

//创建软链接
fn ln(current_path: &PathBuf, params: &[&str]) -> io::Result<()> {
    match params {
        ["-s", from, to] => {
            let mut from_path = current_path.clone();
            from_path.push(from);

            let mut to_path = current_path.clone();
            to_path.push(to);

            #[cfg(target_family = "unix")]
            {
                unix::fs::symlink(from_path, to_path)
            }

            #[cfg(target_family = "windows")]
            {
                windows::fs::symlink_file(from_path, to_path)
            }
        }
        [_] => {
            println!("需要指定两个参数");
            io::Result::Err(io::Error::from(io::ErrorKind::InvalidData))
        }
        [] | [_, _, _, ..] => todo!(),
        &[_, _] => todo!(),
    }
}

fn ls(current_path: &Path) {
    match fs::read_dir(current_path) {
        Err(err) => println!("文件路径读取错误:{}", err),
        Ok(read_dirs) => {
            for read_dir in read_dirs {
                println!("{:?}", read_dir.unwrap().path());
            }
        }
    }
}

fn cd<'a>(mut current_path: &'a mut PathBuf, cd_path: &str) {
    match cd_path {
        "./" => {}
        "../" => {
            current_path.pop();
        }
        (cd_path) => {
            let paths = fs::read_dir(&current_path).unwrap();
            let cd_able = paths
                .filter(|entry| {
                    if let Ok(entry) = entry {
                        let is_dir = entry.path().is_dir();
                        let is_contain =
                            entry.path().file_name().unwrap().to_str().unwrap() == cd_path;
                        is_dir && is_contain
                    } else {
                        false
                    }
                })
                .count()
                == 1;

            if cd_able {
                current_path.push(cd_path);
            } else {
                println!("文件路径不存在:{}/{}", current_path.display(), cd_path);
            }
        }
    }
}

fn rm(mut current_path: &PathBuf, args: &[&str]) {
    let mut current_path = current_path.clone();

    match args {
        ["-rf", arg] => {
            current_path.push(arg);
            if let Err(err) = fs::remove_dir_all(current_path) {
                println!("递归删除出错")
            }
        }
        [file_name] => {
            current_path.push(file_name);
            if let Err(err) = fs::remove_file(current_path) {
                println!("删除文件出错")
            }
        }
        &[] | &[_, _, _, ..] => todo!(),
        &[&_, _] => todo!(),
    }
}

fn process<'a>(current_path: &'a mut PathBuf, line: &str) {
    let lines = line.split_whitespace().collect::<Vec<&str>>();
    //这里要把vec转换为切片再进行match
    match lines.as_slice() {
        ["ls"] => ls(current_path),
        ["cd", path] => cd(current_path, &path),
        ["cat", file_name] => cat(current_path, file_name),
        ["mkdir", params @ ..] => {
            mkdir(current_path, params);
        }
        ["echo", args @ ..] => {
            echo(current_path, args);
        }
        ["touch", file_name] => {
            touch(current_path, file_name);
        }
        ["ln", args @ ..] => {
            ln(current_path, args);
        }
        ["rm", args @ ..] => {
            rm(current_path, args);
        }
        &[] => todo!(),
        &[&_] | &[&_, _] | &[&_, _, _, ..] => todo!(),
    }
}

pub fn main() {
    let mut current_path = PathBuf::new();
    current_path.push(".");
    let mut line = String::new();
    loop {
        println!("zhangjun-fedora:{}#", current_path.to_str().unwrap());
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        process(&mut current_path, &line.trim());
    }
}
