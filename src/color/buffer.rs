use color::sample::*;

pub struct ColorBuffer {
    pub buffer: Vec<SamplePrecision>,
    pub imgx: usize,
    pub imgy: usize,
}

impl ColorBuffer {
    pub fn new(imgx: usize, imgy: usize) -> ColorBuffer {
        let buffer: Vec<SamplePrecision> = Vec::with_capacity(3 * imgx * imgy);

        ColorBuffer {
            buffer: buffer,
            imgx: imgx,
            imgy: imgy,
        }
    }
    pub fn push_color(&mut self, color: ColorSample) {
        self.buffer.extend([color.red, color.green, color.blue].iter());
    }
}

