use std::fmt;
use std::fmt::Formatter;

pub struct Commodity {
    pub id: u8,
    pub name: String,
    pub price: f32,
}
impl fmt::Display for Commodity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:>3} {:>15} {:>15}", self.id, self.name, self.price)
    }
}

pub fn append(all_commodity: &mut Vec<Commodity>, input_line: &String) {
    let mut a = input_line.split(" ");
    let id:u8= a.next().unwrap().parse::<u8>().unwrap();
    let name:String= a.next().unwrap().parse::<String>().unwrap();
    let price:f32=a.next().unwrap().trim().parse::<f32>().unwrap();

    all_commodity.push(Commodity {
        id: id,
        name: name,
        price: price,
    })
}

pub fn display_all_commodities(all_commodities: &Vec<Commodity>) {
    println!("{:>3}{:>15}{:>15}", "编号", "商品名称", "价格");
    all_commodities.iter().for_each(|commodity| {
        println!("{}", commodity);
    })
}
