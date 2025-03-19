#[derive(Debug)]
enum MathError {
    DivideByZero,
    NonPositiveLogarithm,
    NegativeLogarithm,
}

type MathResult = Result<f64, MathError>;

fn div(x: f64, y: f64) -> MathResult {
    if (y == 0.0) {
        Err(MathError::DivideByZero)
    } else {
        Ok(x / y)
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeLogarithm)
    } else {
        Ok(x.sqrt())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}

fn op(x: f64, y: f64) -> f64 {
    match div(x, y) {
        Ok(ratio) => match ln(ratio) {
            Ok(ln) => match sqrt(ln){
                Ok(ln) => ln,
                Err(err) => panic!("Negative logarithm of a result")
            },
            Err(err) => panic!("Negative logarithm of a result"),
        },
        Err(why) => {
            panic!("{:?}", why)
        }
    }
}


pub fn main(){
    println!("{:?}", op(1.0,10.0));
}
