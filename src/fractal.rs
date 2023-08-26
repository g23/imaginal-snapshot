use crate::bitmap::Bitmap;
use crate::colors::{Color, BLACK};
use crate::complex::Complex;
use crate::Params;

fn make_screen_to_complex(
    width: u16,
    height: u16,
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) -> impl Fn(u16, u16) -> Complex<f64> {
    let complex_width = bottom_right.r - top_left.r;
    let complex_height = bottom_right.i - top_left.i; // backwards bc screens count +y as going down
    move |x: u16, y: u16| {
        let w = complex_width * (x as f64) / (width as f64);
        let h = complex_height * (y as f64) / (height as f64);
        let z = Complex::new(w, h);
        top_left + z
    }
}

fn escape_iters(c: Complex<f64>, max_iters: i64) -> Option<i64> {
    let mut count = 0;
    let mut z: Complex<f64> = (0, 0).into();
    while count < max_iters {
        z = z * z + c;

        if z.mag_squared() > 16.0 {
            return Some(count);
        }
        count += 1;
    }
    None // didn't escape
}

pub fn render_mandelbrot<F>(params: Params, palette: F) -> Bitmap
where
    F: Fn(f64, f64) -> Color,
{
    let width = params.width;
    let height = params.height;
    let zoom = params.zoom;
    let center = params.center;
    let max_iters = params.max_iters;
    let gradient_iters = params.gradient_iters.unwrap_or(max_iters);

    let width_scale = (width as f64) / (height as f64);
    let complex_height = 1.0 / zoom;
    let complex_width = complex_height * width_scale;
    let top_left = (
        center.r - complex_width / 2.0,
        center.i + complex_height / 2.0,
    )
        .into();
    let bottom_right = (
        center.r + complex_width / 2.0,
        center.i - complex_height / 2.0,
    )
        .into();

    println!("top_left: {top_left}");
    println!("bottom_right: {bottom_right}");

    let mut img = Bitmap::new(width, height);
    let screen_to_complex = make_screen_to_complex(width, height, top_left, bottom_right);
    let gradient_iters = gradient_iters as f64;
    for y in (0..height).rev() {
        // apparently .bmp stores pixels "bottom up" so need to .rev()
        for x in 0..width {
            let c = screen_to_complex(x, y);
            let escape = escape_iters(c, max_iters).unwrap_or(max_iters);
            if escape == max_iters {
                img.push_pixel(BLACK.into());
            } else {
                let c = palette(escape as f64, gradient_iters);
                img.push_pixel(c.into());
            }
        }
    }
    img
}

pub fn render_color_palette<F>(width: u16, height: u16, palette: F) -> Bitmap
where
    F: Fn(f64, f64) -> Color,
{
    let max_iters = (width + height) as f64;
    let mut img = Bitmap::new(width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            img.push_pixel(palette((x + y) as f64, max_iters / 4.0).into())
        }
    }
    img
}
