use crate::commodity::Commodity;
use std::ops::Mul;

pub struct CheckItem {
    id: u8,
    quantity: u8,
}
pub fn append_check(check_items: &mut Vec<CheckItem>, line: &String) {
    let mut sp = line.split(" ");
    let id = sp.next().unwrap().parse::<u8>().unwrap();
    let quantity = sp.next().unwrap().trim().parse::<u8>().unwrap();
    check_items.push(CheckItem { id, quantity });
}

pub fn check_check(commodities: &Vec<Commodity>, check_items: &Vec<CheckItem>) {
    println!(
        "{:=>3}{:=>15}{:=>15}{:=>3}{:=>15}",
        "编号", "商品名称", "单价", "数量", "价格"
    );
    for x in check_items {
        let commodity = find(commodities, x.id);
        match commodity {
            None => {}
            Some(got_commodity) => {
                println!(
                    "{:>3}{:>15}{:>22}{:>3}{:>15}",
                    got_commodity.id,
                    got_commodity.name,
                    got_commodity.price,
                    x.quantity,
                    (x.quantity as f32).mul(got_commodity.price)
                );
            }
        }
    }
}

pub fn find<'a>(commodities: &'a Vec<Commodity>, id: u8) -> Option<&'a Commodity> {
    commodities.iter().filter(|c| c.id == id).next()
}
