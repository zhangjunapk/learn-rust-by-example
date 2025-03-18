use std::fmt::Debug;

struct UsbBus<'a, T> {
    usb_device: Vec<Box<dyn Usb<'a, T> + 'a>>,
}

struct UsbRef<'a, T: 'a>(&'a mut dyn Usb<'a, T>);

impl<'a, T: Debug + 'a> Usb<'a, T> for UsbRef<'a, T> {
    fn connect(&mut self, usb_bus: &'a UsbBus<'a, T>) {
        self.0.connect(usb_bus);
    }

    fn receive(&self, data: T) {
        self.0.receive(data);
    }

    fn disconnect(&self) {
        self.0.disconnect();
    }

    fn info(&self) {
        self.0.info();
    }
}

#[derive(Debug)]
struct Point(i32, i32);

struct Computer<'a, T: Debug + 'a> {
    name: String,
    price: f64,
    brand: String,
    usb_port_num: i8,
    usb_bus:  UsbBus<'a, T>,
}

impl<'a, T: Debug + 'a> Computer<'a, T> {
    pub fn new() -> Self {
        Self {
            name: String::from("长城笔记本"),
            price: 900f64,
            brand: String::from("长城"),
            usb_port_num: 2,
            usb_bus: UsbBus {
                usb_device: Vec::new(),
            },
        }
    }
    pub fn power_on(&self) {
        println!("{}开始开机", self.name)
    }
    pub fn connect_usb(&mut self, usb: &'a mut (impl Usb<'a, T> + 'a)) {
        if (self.usb_bus.usb_device.len() == self.usb_port_num as usize) {
            println!("达到usb支持设备上限，无法继续插入usb设备");
            return;
        }
        usb.info();
        //usb.connect(&self.usb_bus);
        &self.usb_bus.usb_device.push(Box::new(UsbRef(usb)));

    }
    pub fn disconnect(&mut self, index: u8) {
        if (index >= self.usb_bus.usb_device.len() as u8) {
            println!("指定usb设备不存在");
            return;
        }
        self.usb_bus
            .usb_device
            .get(index as usize)
            .unwrap()
            .disconnect();
        self.usb_bus.usb_device.remove(index as usize);
    }

    pub fn power_off(&self) {
        println!("电脑开始关机");
        println!("开始移除usb设备");
        for x in &self.usb_bus.usb_device {
            x.disconnect();
        }
        println!("usb设备移除完成，开始断电");
    }
}

trait Usb<'a, T>
where
    T: Debug,
{
    fn connect(&mut self, usb_bus: &'a UsbBus<'a, T>);
    fn receive(&self, data: T);
    fn disconnect(&self);
    fn info(&self);
}

struct Keyboard<'a, T> {
    brand: String,
    price: f64,
    name: String,
    usb_bus: UsbBus<'a, T>,
}

impl Usb<'_, char> for Keyboard<'_, char> {
    fn connect(&mut self, usb_bus: &UsbBus<char>) {
        self.info();

        println!("鼠标已经连接")
    }

    fn receive(&self, data: char) {
        println!("按下按键:{}", data)
    }

    fn disconnect(&self) {
        println!("键盘断开连接")
    }

    fn info(&self) {
        println!("键盘信息:");
        println!("品牌:{}", self.brand);
        println!("名称:{}", self.name);
        println!("价格:{}", self.price);
    }
}

struct Mouse<'a, T> {
    brand: &'static str,
    price: f64,
    name: &'static str,
    usb_bus_ref: Option<&'a UsbBus<'a, T>>,
}

impl<'a> Usb<'a, Point> for Mouse<'a, Point> {
    fn connect(&mut self, usb_bus: &'a UsbBus<'a, Point>) {
        println!("鼠标连接成功");
        self.usb_bus_ref = Some(usb_bus);
    }

    fn receive(&self, data: Point) {
        println!("鼠标点击位置:x:{},y:{}", data.0, data.1)
    }

    fn disconnect(&self) {
        println!("鼠标断开连接")
    }

    fn info(&self) {
        println!("鼠标信息:");
        println!("品牌:{:?}", self.brand);
        println!("价格:{}", self.price);
        print!("名称:{}", self.name);
    }
}

fn main() {
    //这里先调整定义顺序，保证Computer销毁前，mouse的生命周期一致存在l
    let mut mouse: Mouse<'_, Point> = Mouse {
        name: ("b65系列a款鼠标"),
        price: 30f64,
        brand: ("雷蛇"),
        usb_bus_ref: None,
    };
    let mut computer: Computer<Point> = Computer::new();

    computer.power_on();
    computer.connect_usb(&mut mouse);
    // mouse.receive(Point(20, 30));
    computer.power_off();
}
