pub fn main() {
    let shadowed_binding=1;
    {
        println!("被遮蔽前:{}",shadowed_binding);
        let shadowed_binding="abc";
        println!("遮蔽后:{}",shadowed_binding);
    }
    println!("外部区域，未遮蔽:{}",shadowed_binding);
    let shadowed_binding=2;
    println!("外部区域被遮蔽:{}",shadowed_binding);
}
