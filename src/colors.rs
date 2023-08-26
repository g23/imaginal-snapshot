use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64);

pub const BLACK: Color = Color(0.0, 0.0, 0.0);
pub const BLUE: Color = Color(0.0, 0.0, 1.0);
pub const GREEN: Color = Color(0.0, 1.0, 0.0);
pub const CYAN: Color = Color(0.0, 1.0, 1.0);
pub const RED: Color = Color(1.0, 0.0, 0.0);
pub const MAGENTA: Color = Color(1.0, 0.0, 1.0);
pub const YELLOW: Color = Color(1.0, 1.0, 0.0);
pub const WHITE: Color = Color(1.0, 1.0, 1.0);

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
    ($a:expr, $b: expr, $($rest:expr),+) => {
        max!(max!($a, $b), $($rest),+)
    };
}

impl Color {
    pub fn normalize(&self) -> Color {
        let Color(r, g, b) = self;
        let m = max!(r, g, b);
        Color(r / m, g / m, b / m)
    }

    pub fn palette_fn(&self) -> impl Fn(f64, f64) -> Color {
        let this = self.clone();
        move |_, _| this
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Color(r, g, b)
    }
}

impl From<Color> for Rgb {
    fn from(Color(r, g, b): Color) -> Self {
        Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl From<Rgb> for Color {
    fn from(Rgb(r, g, b): Rgb) -> Self {
        Color((r as f64) / 255.0, (g as f64) / 255.0, (b as f64) / 255.0)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, Color(r, g, b): Color) -> Self::Output {
        Color(self.0 + r, self.1 + g, self.2 + b)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, Color(r, g, b): Color) -> Self::Output {
        Color(self * r, self * g, self * b)
    }
}

pub fn blend<F, G>(f: F, g: G) -> impl Fn(f64, f64) -> Color
where
    F: Fn(f64, f64) -> Color,
    G: Fn(f64, f64) -> Color,
{
    move |iters, max_iters| {
        let a = f(iters, max_iters);
        let b = g(iters, max_iters);
        let lambda = iters / max_iters;
        (1.0 - lambda) * a + lambda * b
    }
}

pub fn glue<F, G>(f: F, g: G, r: f64) -> impl Fn(f64, f64) -> Color
where
    F: Fn(f64, f64) -> Color,
    G: Fn(f64, f64) -> Color,
{
    move |iters, max_iters| {
        let lambda = iters / max_iters;
        if lambda < r {
            f(iters / r, max_iters)
        } else {
            g((iters - r * max_iters) / (1.0 - r), max_iters)
        }
    }
}

pub fn wrapping<F>(f: F) -> impl Fn(f64, f64) -> Color
where
    F: Fn(f64, f64) -> Color,
{
    move |iters, max_iters| {
        let parity = ((iters as i64) / (max_iters as i64)) % 2;
        let iters = iters % max_iters;
        if parity == 0 {
            // to see if in reverse
            f(iters, max_iters)
        } else {
            f(max_iters - iters, max_iters)
        }
    }
}

pub fn rainbow_palette() -> impl Fn(f64, f64) -> Color {
    let g1 = blend(BLACK.palette_fn(), RED.palette_fn());
    let g2 = blend(RED.palette_fn(), YELLOW.palette_fn());
    let g3 = blend(YELLOW.palette_fn(), GREEN.palette_fn());
    let g4 = blend(GREEN.palette_fn(), GREEN.palette_fn());
    let g5 = blend(GREEN.palette_fn(), CYAN.palette_fn());
    let g6 = blend(CYAN.palette_fn(), BLUE.palette_fn());
    let g7 = blend(BLUE.palette_fn(), MAGENTA.palette_fn());
    let g8 = blend(MAGENTA.palette_fn(), WHITE.palette_fn());

    let gl1 = glue(g1, g2, 0.5);
    let gl2 = glue(g3, g4, 0.5);
    let gl3 = glue(g5, g6, 0.5);
    let gl4 = glue(g7, g8, 0.5);

    let gl12 = glue(gl1, gl2, 0.5);
    let gl34 = glue(gl3, gl4, 0.5);

    let gl1234 = glue(gl12, gl34, 0.5);
    wrapping(gl1234)
}
