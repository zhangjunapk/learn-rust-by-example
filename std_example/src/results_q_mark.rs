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

fn op(x: f64, y: f64) -> MathResult {
    /*
    //这样用更方便
    Some(x)
    .and_then(|x| Some(x / y))
    .and_then(|res| Some(res.ln()))
    .and_then(|res| Some(res.sqrt()))*/
    let div = div(x, y)?;
    let ln = ln(div)?;
    sqrt(ln)
}

pub fn main() {
    println!("{:?}", op(1.0, 10.0));
}
