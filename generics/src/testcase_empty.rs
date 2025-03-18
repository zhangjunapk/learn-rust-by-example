struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(t: &T) -> &'static str {
    "红色"
}
fn blue<T: Blue>(t: &T) -> &'static str {
    "蓝色"
}
pub fn main() {
    let a = Cardinal;
    let b = BlueJay;
    let c = Turkey;
    println!("a:{}", red(&a));
    println!("b:{}", blue(&b));
    // let d=blue(&c);
}
