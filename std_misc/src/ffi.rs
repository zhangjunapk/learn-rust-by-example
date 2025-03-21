use std::fmt;
use std::fmt::Formatter;

#[cfg(target_family = "windows")]
#[link(name = "msvcrt")]
extern{
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

#[cfg(target_family = "unix")]
#[link(name = "m")]
unsafe extern {
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

pub fn main() {
    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };
    println!("{:?}的平方根是:{:?}", z, z_sqrt);
    println!("调用封装了不安全的操作api cos({:?})={:?}", z, cos(z));
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
