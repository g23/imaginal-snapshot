// def of complex numbers
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub trait Number<T>: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {}
impl<T> Number<T> for T where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {}

#[derive(Copy, Clone, Debug)]
pub struct Complex<T: Number<T>> {
    pub r: T,
    pub i: T,
}

impl<T: Number<T>> Complex<T> {
    pub fn new(r: T, i: T) -> Self {
        Complex { r, i }
    }

    pub fn mag_squared(&self) -> T {
        self.r * self.r + self.i * self.i
    }
}

impl<T: Number<T>> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self.r + rhs.r, self.i + rhs.i)
    }
}

impl<T: Number<T>> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self.r - rhs.r, self.i - rhs.i)
    }
}

impl<T: Number<T>> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        let r = self.r * rhs.r - self.i * rhs.i;
        let i = self.r * rhs.i + self.i * rhs.r;
        Complex::new(r, i)
    }
}

// now some conversion hacks
macro_rules! impl_complex_from {
    ($a: ty, $b: ty) => {
        impl From<Complex<$a>> for Complex<$b> {
            fn from(z: Complex<$a>) -> Self {
                Complex::new(z.r as $b, z.i as $b)
            }
        }
    };
}

impl_complex_from!(u16, f64);
impl_complex_from!(u32, f64);

// this doesn't work so well...
impl<T: Number<T>> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex::new(value.0, value.1)
    }
}
// so hacks
macro_rules! impl_tuple_from {
    ($a_prim: ty, $b_prim: ty) => {
        impl From<($a_prim, $a_prim)> for Complex<$b_prim> {
            fn from(value: ($a_prim, $a_prim)) -> Self {
                Complex::new(value.0 as $b_prim, value.1 as $b_prim)
            }
        }
    };
}

impl_tuple_from!(i32, f32);
impl_tuple_from!(i64, f64);

// so more hacks
macro_rules! impl_display {
    ($prim: ty, $prim_zero: literal) => {
        impl Display for Complex<$prim> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.i < $prim_zero {
                    f.write_str(&format!("{} - {}i", self.r, self.i.abs())[..])
                } else if self.i == $prim_zero {
                    f.write_str(&format!("{}", self.r)[..])
                } else if self.r == $prim_zero {
                    f.write_str(&format!("{}i", self.i)[..])
                } else {
                    f.write_str(&format!("{} + {}i", self.r, self.i)[..])
                }
            }
        }
    };
}

impl_display!(i32, 0);
impl_display!(i64, 0);
impl_display!(f32, 0.0);
impl_display!(f64, 0.0);

// seemingly, clap crate has problems (or idk how to use it) with parsing negative numbers...
// so need a tuple from...
impl From<String> for Complex<f64> {
    fn from(s: String) -> Self {
        if let Some((r, i)) = s.split_once(",") {
            match (r.find("("), i.find(")")) {
                (Some(ridx), Some(iidx)) if ridx + 1 < r.len() => {
                    let (_, r) = r.split_at(ridx + 1);
                    let (i, _) = i.split_at(iidx);
                    let r = r.trim().parse().unwrap_or(0.0);
                    let i = i.trim().parse().unwrap_or(0.0);
                    Complex::new(r, i)
                }
                _ => (0.0, 0.0).into(),
            }
        } else if let Some((r, i)) = s.split_once("+") {
            let r = r.trim().parse().unwrap_or(0.0);
            if let Some(iidx) = i.find("i") {
                let (i, _) = i.split_at(iidx);
                let i = i.trim().parse().unwrap_or(0.0);
                (r, i).into()
            } else {
                (r, 0.0).into()
            }
        } else if let Some((r, i)) = s.split_once("-") {
            let r = r.trim().parse().unwrap_or(0.0);
            if let Some(iidx) = i.find("i") {
                let (i, _) = i.split_at(iidx);
                let i = i.trim().parse().unwrap_or(0.0);
                (r, i).into()
            } else {
                (r, 0.0).into()
            }
        } else {
            (0.0, 0.0).into()
        }
    }
}
