use practice1::check::CheckItem;
use practice1::check::{append_check, check_check};
use practice1::commodity::{append, display_all_commodities, Commodity};
use std::io::stdin;

fn main() {
    let user_input = &mut String::new();
    let mut commodities: Vec<Commodity> = Vec::new();
    let mut check_items: Vec<CheckItem> = Vec::new();

    loop {
        println!("欢迎来到广岛超市收银台");
        println!("1:显示所有当前商品 2:添加商品 3:开始记账收银");
        &user_input.clear();
        stdin().read_line(user_input).unwrap();
        let input = user_input.trim();

        if (input.eq("1")) {
            display_all_commodities(&commodities);
        } else if (input.eq("2")) {
            println!("输入商品信息 编号 名称 单价");
            &user_input.clear();
            stdin().read_line(user_input).unwrap();
            append(&mut commodities, user_input)
        } else if (input.eq("3")) {
            check(&mut check_items, &commodities)
        }
    }
}

fn check(check_items: &mut Vec<CheckItem>, commodities: &Vec<Commodity>) {
    println!("输入商品信息 编号 数量");
    loop {
        let user_input = &mut String::new();
        user_input.clear();
        stdin().read_line(user_input).unwrap();
        if (!user_input.trim().eq("q")) {
            append_check(check_items, user_input);
        } else {
            println!("添加完成，正在计算金额");
            check_check(&commodities, &check_items);
            break;
        }
    }
}
