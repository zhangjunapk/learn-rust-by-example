use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[cfg(targetos = "windows")]
fn command() -> Command {
    let mut cmd = Command::new("powershell");
    cmd.arg("-Command")
        .arg("$input | Measure-Object -Line -Word -Character");
    cmd
}

#[cfg(target_os = "linux")]
fn command() -> Command {
    Command::new("wc")
}

pub fn main() {
    let mut command = command();
    let child = match command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Ok(process) => process,
        Err(error) => panic!("failed to spawn \"wc\" process: {}", error),
    };
    match child.stdin.unwrap().write_all("you love".as_bytes()) {
        Err(why) => panic!("failed to write to \"wc\" stdin: {}", why),
        Ok(_) => println!("sent \"adff\" to wc"),
    }
    let mut s = String::new();
    if let Ok(ok) = child.stdout.unwrap().read_to_string(&mut s) {
        println!("wc执行结果:{},大小:{}", s, ok);
    }
}
