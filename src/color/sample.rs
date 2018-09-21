use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

pub type SamplePrecision = f64;

#[derive(Debug, Copy, Clone)]
pub struct ColorSample {
    pub red: SamplePrecision,
    pub green: SamplePrecision,
    pub blue: SamplePrecision,
}

impl ColorSample {
    pub const BLACK: ColorSample = ColorSample {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const WHITE: ColorSample = ColorSample {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
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

impl AddAssign for ColorSample {
    fn add_assign(&mut self, other: ColorSample) {
        *self = *self + other;
    }
}

impl Div<usize> for ColorSample {
    type Output = ColorSample;

    fn div(self, count: usize) -> ColorSample {
        let recip = (count as SamplePrecision).recip();
        self * recip
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

impl Mul<ColorSample> for ColorSample {
    type Output = ColorSample;

    fn mul(self, other: ColorSample) -> ColorSample {
        ColorSample {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl MulAssign<ColorSample> for ColorSample {
    fn mul_assign(&mut self, other: ColorSample) {
        *self = *self * other;
    }
}

impl MulAssign<SamplePrecision> for ColorSample {
    fn mul_assign(&mut self, scalar: SamplePrecision) {
        *self = *self * scalar;
    }
}
