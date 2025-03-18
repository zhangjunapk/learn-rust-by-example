struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0*365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_audlt(age: &Years) -> bool {
    age.0 >= 18
}
pub fn main() {
    let age = Years(25);
    let days = age.to_days();
    println!("是否是成年人:{}", is_audlt(&age));
    println!("是否是成年人:{}",is_audlt(&days.to_years()));
    // println!("是否是成年人:{}",is_audlt(&days));

    let years=Years(42);
    let years_as_primitive_1:i64=years.0;
    let Years(years_as_primitive_2)=years;

}
