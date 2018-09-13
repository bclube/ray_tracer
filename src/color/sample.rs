use std::ops::{Add, Mul};

pub type SamplePrecision = f64;

#[derive(Debug, Copy, Clone)]
pub struct ColorSample {
    pub red: SamplePrecision,
    pub green: SamplePrecision,
    pub blue: SamplePrecision,
}

impl Add for ColorSample {
    type Output = ColorSample;

    fn add(self, other: ColorSample) -> ColorSample {
        ColorSample {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul<SamplePrecision> for ColorSample {
    type Output = ColorSample;

    fn mul(self, scalar: SamplePrecision) -> ColorSample {
        ColorSample {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Mul<ColorSample> for SamplePrecision {
    type Output = ColorSample;

    fn mul(self, color_sample: ColorSample) -> ColorSample {
        color_sample * self
    }
}
