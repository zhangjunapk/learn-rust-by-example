use std::collections::HashMap;

//实现特性后，就能作为hashmap的key
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    name: &'a str,
    password: &'a str,
}
struct AccountInfo<'a> {
    sex: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn login<'a,'b:'a>(
    accounts: &'a HashMap<Account<'a>, AccountInfo<'a>>,
    name: &'b str,
    password: &'b str,
) -> Option<&'a AccountInfo<'a>> {
    let temp_account = Account { name, password };
    if (accounts.contains_key(&temp_account)) {
        accounts.get(&temp_account)
    } else {
        None
    }
}

pub fn main() {
    let mut accounts = HashMap::new();
    accounts.insert(
        Account {
            name: "john",
            password: "aaaa",
        },
        AccountInfo {
            sex: "男性",
            email: "john@example.com",
        },
    );

    if let Some(account_info) = login(&accounts, "john", "aaaa") {
        println!(
            "登录成功，获取到用户信息:性别:{},邮箱:{}",
            account_info.sex, account_info.email
        );
    } else {
        println!("找不到用户信息，或者用户名和密码不匹配")
    }
}
