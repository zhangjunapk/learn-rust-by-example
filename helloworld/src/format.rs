use std::fmt;
use std::fmt::Formatter;

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //北纬
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        //东经
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}:{:.3}{} {:.3}{}",
            self.name, self.lat, lat_c, self.lon, lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({},{},{} 0x{:0>6X})",
            self.red,
            self.green,
            self.blue,
            (65536 * self.red as u64) + (256 * self.green as u64) + self.blue as u64
        )
    }
}

pub fn main() {
    let city = City {
        name: "也门-萨那",
        lat: 15.369445,
        lon: 44.191006,
    };
    println!("{}", city);

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{:?}", color);
        println!("{}",color);
    }
}
