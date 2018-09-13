pub type SamplePrecision = f64;

#[derive(Debug, Copy, Clone)]
pub struct ColorSample {
    pub red: SamplePrecision,
    pub green: SamplePrecision,
    pub blue: SamplePrecision,
}
