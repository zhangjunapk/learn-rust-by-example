use std::fmt::Debug;

// 14.7 新类型惯用法：封装日志级别
#[derive(Debug)]
struct Info(String);
#[derive(Debug)]
struct Error(String);

// 14.3 特质 & 14.8.2 关联类型：定义 Loggable trait
trait Loggable {
    type Level: Debug; // 关联类型
    fn level(&self) -> Self::Level;
    fn message(&self) -> String;
}

// 14.2 实现：为 Info 和 Error 实现 Loggable
impl Loggable for Info {
    type Level = Info;
    fn level(&self) -> Self::Level {
        Info(self.0.clone())
    }
    fn message(&self) -> String {
        format!("Recorded {:?}", self.0)
    }
}

impl Loggable for Error {
    type Level = Error;
    fn level(&self) -> Self::Level {
        Error(self.0.clone())
    }
    fn message(&self) -> String {
        format!("Recorded {:?}", self.0)
    }
}

// 14.1 泛型 & 14.4 约束 & 15.4.4 结构体生命周期
#[derive(Debug)]
struct Logger<'a, T>
where
    T: Debug + Clone + 'a, // 14.6 Where 分句 & 15.4.6 生命周期约束
{
    records: Vec<(T, Box<dyn Loggable<Level=()>>)>, // 14.9 虚类型参数（Box<dyn>）
    static_ref: Option<&'static str>,     // 15.4.8 静态生命周期
    phantom: std::marker::PhantomData<&'a T>, // 15.4.4 生命周期绑定
}

// 15.1 RAII：实现 Drop
impl<'a, T> Drop for Logger<'a, T>
where
    T: Debug + Clone + 'a,
{
    fn drop(&mut self) {
        println!("Logger dropped, total records: {}", self.records.len());
    }
}

// 14.2 实现 & 14.5 多重约束
impl<'a, T> Logger<'a, T>
where
    T: Debug + Clone + 'a,
{
    fn new() -> Self {
        Logger {
            records: Vec::new(),
            static_ref: None,
            phantom: std::marker::PhantomData,
        }
    }

    // 15.2 所有权和移动：记录值
    fn log(&mut self, value: T, level: impl Loggable) {
        self.records.push((value, Box::new(level)));
    }

    // 15.3 借用：记录引用
    fn log_ref(&mut self, value: &'a T, level: impl Loggable) {
        self.records.push((value.clone(), Box::new(level)));
    }

    // 15.4.8 静态：记录静态数据
    fn log_static(&mut self, value: &'static str) {
        self.static_ref = Some(value);
    }

    // 14.4 测试空约束（仅打印）
    fn print(&self) {
        for (val, level) in &self.records {
            println!("{:?}: {}", level.level(), level.message());
        }
        if let Some(s) = self.static_ref {
            println!("Static: Recorded {:?}", s);
        }
    }
}

// 15.2.2 部分移动：自定义结构体
#[derive(Debug, Clone)]
struct Custom {
    x: i32,
    y: i32, // 可部分移动
}

fn main() {
    let mut logger = Logger::new();

    // 15.2 所有权：移动值
    let i = 42;
    logger.log(i, Info("42".to_string()));

    // 15.3.1 可变性 & 15.4.1 显式注解：借用
    let s = String::from("hello");
    // logger.log_ref(&s, Error("hello".to_string()));

    // 15.2.2 部分移动
    let c = Custom { x: 10, y: 20 };
    // logger.log(c, Info("Custom".to_string()));
    // println!("c.x: {}", c.x); // 错误：c 已移动

    // 15.4.8 静态生命周期
    logger.log_static("static data");

    logger.print();
    // RAII：logger 离开作用域，调用 drop
}