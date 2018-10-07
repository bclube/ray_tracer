use color::sample::*;

pub struct ColorBuffer {
    pub buffer: Vec<SamplePrecision>,
    pub sample_counts: Vec<usize>,
    pub imgx: usize,
    pub imgy: usize,
}

impl ColorBuffer {
    pub fn new(imgx: usize, imgy: usize) -> ColorBuffer {
        let buffer: Vec<SamplePrecision> = vec![0.0; 3 * imgx * imgy];
        let sample_counts: Vec<usize> = vec![0; imgx * imgy];

        ColorBuffer {
            buffer: buffer,
            sample_counts: sample_counts,
            imgx: imgx,
            imgy: imgy,
        }
    }

    pub fn add_color(&mut self, x: usize, y: usize, color: ColorSample) {
        let idx = y * self.imgx + x;
        let idx2 = idx * 3;
        self.sample_counts[idx] += 1;
        self.buffer[idx2] += color.red;
        self.buffer[idx2 + 1] += color.green;
        self.buffer[idx2 + 2] += color.blue;
    }
}
