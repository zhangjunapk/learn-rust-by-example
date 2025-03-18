enum WebEvent {
    PageLoad,
    PageUpload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum ThisIsEnumOfMathOperation {
    Add,
    Subtract,
}


impl ThisIsEnumOfMathOperation {

    fn info(){
        println!("全局方法");
    }

    //每个枚举的方法
    fn run(&self, a: f32, b: f32) {
        match self {
            //这里的Self也是别名，
            Self::Add => println!("Add operation add:{}", a + b),
            Self::Subtract => println!("Subtract operation subtract:{}", a - b),
        }
    }
}

type Operation = ThisIsEnumOfMathOperation;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("页面已加载"),
        WebEvent::PageUpload => println!("页面已卸载"),
        WebEvent::KeyPress(c) => println!("按下按键{}", c),
        WebEvent::Paste(string) => println!("粘贴内容{}", string),
        WebEvent::Click { x, y } => {
            println!("点击事件:x:{},y:{}", x, y);
        }
    }
}

pub fn main() {
    let page_load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 332, y: 256 };
    let pressed = WebEvent::KeyPress('z');
    let paste = WebEvent::Paste(String::from("google"));
    let upload = WebEvent::PageUpload;
    inspect(page_load);
    inspect(click);
    inspect(pressed);
    inspect(paste);
    inspect(upload);

    let x = Operation::Add;
    x.run(1f32,2f32);
    Operation::info();
}
