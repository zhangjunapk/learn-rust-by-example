use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "110" => "别动，人民警察🔫",
        "119" => "别动，我帮你去去火🎆",
        "120" => "别动，我帮你检查检查身体🛂",
        _ => "你是谁?",
    }
}

pub fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("DD", "110");
    contacts.insert("CC", "119");
    contacts.insert("Angel", "120");
    contacts.insert("Monkey", "911");

    if contacts.contains_key("DD") {
        println!("正在打电话");
        let number = contacts.get("DD").unwrap();
        let res = call(number);
        println!("{}", res);
    }
    if contacts.contains_key("CC") {
        println!("正在打电话");
        let number = contacts.get("CC").unwrap();
        let res = call(number);
        println!("{}", res);
    }
    contacts.remove("Monkey");

    for (key, value) in contacts.iter() {
        println!("正在呼叫:{}: 电话号码:{}", key, value);
    }
}
