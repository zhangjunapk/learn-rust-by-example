#[derive(Copy, Clone)]
struct Person {
    job: Option<Job>,
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    //这里如果有一层没能获取到，返回值会是None
    //代码执行也不会panic
    //相当于java的Option
    fn area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

pub fn main() {
    let john_son = Person {
        job: Some(Job {
            phone_number: Option::<PhoneNumber>::Some(PhoneNumber {
                area_code: Some(56),
                number: 151415255,
            }),
        }),
    };
    let area_code = john_son.area_code();
    if let Some(ref code) = area_code {
        println!("got job area code:{}", code);
    }
}
