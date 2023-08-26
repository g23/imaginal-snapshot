mod bitmap;
mod colors;
mod complex;
mod fractal;

use colors::rainbow_palette;
use complex::Complex;
use fractal::{render_color_palette, render_mandelbrot};

use clap::Parser;

use std::error::Error;
use std::fs;
use std::string::ToString;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Params {
    /// pixel width of the image
    #[arg(short, long, default_value_t = 300)]
    pub width: u16,
    /// pixel height of the image
    #[arg(long, default_value_t = 200)]
    pub height: u16,
    /// zoom level
    #[arg(short, long, default_value_t = 1.0)]
    pub zoom: f64,
    /// center of the window
    #[arg(short, long, default_value_t = Complex::new(-0.75, 0.5))]
    pub center: Complex<f64>,
    /// maximum iterations for escape checking
    #[arg(short, long, default_value_t = 1000)]
    pub max_iters: i64,
    /// controls the coloring, defaults to max_iters
    #[arg(short, long)]
    pub gradient_iters: Option<i64>,
}

impl ToString for Params {
    fn to_string(&self) -> String {
        if self.gradient_iters.is_some() {
            format!(
                "-w {w} --height {h} -z {z} -c '{c:?}' -m {m} -g {g}\n",
                w = self.width,
                h = self.height,
                z = self.zoom,
                c = (self.center.r, self.center.i),
                m = self.max_iters,
                g = self.gradient_iters.unwrap()
            )
        } else {
            format!(
                "-w {w} --height {h} -z {z} -c '{c:?}' -m {m}\n",
                w = self.width,
                h = self.height,
                z = self.zoom,
                c = (self.center.r, self.center.i),
                m = self.max_iters
            )
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let params = Params::parse();
    let params_text = params.to_string();

    let width = params.width;
    let height = params.height;

    /*
    println!("rendering a {width} x {height} color-palette.bpm");
    let palette = rainbow_palette();
    let img = render_color_palette(width, height, palette);
    save_image("color-palette.bmp".to_string(), width, height, img);
    return;
    */

    let palette = rainbow_palette();

    println!("Taking a {width} x {height} picture of the imaginal realm");

    let start = SystemTime::now();
    let img = render_mandelbrot(params, palette);
    let elapsed = start.elapsed()?;
    println!("time rendering: {elapsed:?}");

    println!("saving to a file...");

    let start = SystemTime::now();
    // figure out the filename
    let mut pic_number = 0;
    while let Ok(_) = fs::metadata(format!("snapshot-{pic_number}.bmp")) {
        pic_number += 1;
    }
    let file_name = format!("snapshot-{pic_number}.bmp");
    img.save(file_name)?;
    // also save the params in a .txt file
    fs::write(&format!("params-{pic_number}.txt")[..], &params_text[..])?;

    let elapsed = start.elapsed()?;
    println!("saved to snapshot-{pic_number}.bmp and params-{pic_number}.txt in: {elapsed:?}");
    Ok(())
}
