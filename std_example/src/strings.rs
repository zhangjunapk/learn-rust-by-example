use std::str;

pub fn main() {
    let pangram: &'static str = "a b c d e f g";
    println!("打印字母:{}", pangram);
    println!("开始字母逆序");
    //按照空格分割并且逆序
    for vin in pangram.split_whitespace().rev() {
        println!("> {}", vin);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    //去重
    //delete duple
    chars.dedup();
    println!("字符数组:{:?}", chars);

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push(',');
    }
    println!("新建的字符串:{:?}", string);

    let chars_to_trim = &[' ', ','];
    let after_trim = string.trim_matches(chars_to_trim);
    println!("修剪后的字符串:{}", after_trim);

    let say = String::from("rust喜欢螃蟹");
    let replaced = say.replace("螃蟹", "java");
    println!("{}", say);
    println!("{}", replaced);

    //十六进制转义
    let byte_e = "转移字符 \x52 \x75";
    println!("{}", byte_e);

    let unicode = "\u{211D}";
    println!("unicode转移:{}", unicode);

    let character_name = "\"大写R\"";

    println!("character name:{}", character_name);
    let long_string = "字符串字面值
                        可以跨越多行。
                        这里的换行和缩进 ->\
                        <- 也可以被转义！";
    println!("{}", long_string);

    //r开头表示原样
    let raw_str = r"转移试试能否起作用\x3F \u{211D}";
    println!("{}", raw_str);

    let mark = r#"然后我说："无处可逃！""#;
    println!("{}", mark);

    let mark = r"然后我说：无处可逃！";
    println!("{}", mark);

    let somany = r###"这里面会有很多井   # '# "###;
    println!("{}", somany);

    byte_array();
}

fn byte_array() {
    //这里必须是asc2编码
    let bytestring: &[u8] = b"this is content";
    println!("字节打印:{:?}", bytestring);

    //这里十六进制的也能够支持
    let escaped = b"x52x75x73 as byte";
    println!("escaped={:?}", escaped);

    //先原样使用，再转换为字节
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("raw_bytestring={:?}", raw_bytestring);

    if let Ok(str) = str::from_utf8(raw_bytestring) {
        println!("转换为文本:{}", str);
    }

    let shift_jis = b"\x82\xe6\xa8";
    match str::from_utf8(shift_jis) {
        Ok(str) => println!("不可能哦"),
        Err(err) => {
            println!("这就对了，转换失败:{}", err)
        }
    }
}
