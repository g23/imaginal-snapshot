use crate::colors::Rgb;

use std::fs::File;
use std::io::Write;

pub struct Bitmap {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

impl Bitmap {
    pub fn new(width: u16, height: u16) -> Self {
        Bitmap { width, height, pixels: vec![] }
    }
    
    pub fn push_pixel(&mut self, Rgb(r, g, b): Rgb) {
        self.pixels.push(b);
        self.pixels.push(g);
        self.pixels.push(r);
    }
    
    pub fn save(mut self, file_name: String) -> std::io::Result<usize> {
        let file_size: u32 = (self.height as u32) * (self.width as u32) * 3 + 26;

        let mut bmp = vec![];

        bmp.push(b'B'); bmp.push(b'M');
        // 2. size
        bmp.append(&mut file_size.to_le_bytes().to_vec());
        //bmp.push(0); bmp.push(0); bmp.push(0); bmp.push(0);
        // 6. reserved
        bmp.push(0);
        bmp.push(0);
        bmp.push(0);
        bmp.push(0);
        // 10. offset
        bmp.push(26);
        bmp.push(0);
        bmp.push(0);
        bmp.push(0);
        // 14. size of header
        bmp.push(12);
        bmp.push(0);
        bmp.push(0);
        bmp.push(0);
        // 18. width
        bmp.append(&mut self.width.to_le_bytes().to_vec());
        // 20. height
        bmp.append(&mut self.height.to_le_bytes().to_vec());
        // 22. color planes, must be 1
        bmp.push(1);
        bmp.push(0);
        // 24. bits per pixel
        bmp.push(24);
        bmp.push(0);
        // 26. the data?
        bmp.append(&mut self.pixels);
        
        let mut f = File::create(file_name)?;
        f.write(bmp.as_slice())
    }
}

