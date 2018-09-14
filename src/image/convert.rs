use std::u8;
use std::u16;
use color::sample::*;

const EIGHT_BIT_MAX: SamplePrecision = (u8::MAX as SamplePrecision) + 1.0 - 1e-6;
const SIXTEEN_BIT_MAX: SamplePrecision = (u16::MAX as SamplePrecision) + 1.0 - 1e-6;

#[inline(always)]
fn clamp_color(color: SamplePrecision) -> SamplePrecision {
    color.min(1.0).max(0.0)
}

#[inline(always)]
pub fn to_eight_bits(color: SamplePrecision) -> u8 {
    let color = clamp_color(color);
    (EIGHT_BIT_MAX * color).trunc() as u8
}

#[inline(always)]
pub fn to_sixteen_bits(color: SamplePrecision) -> u16 {
    let color = clamp_color(color);
    (SIXTEEN_BIT_MAX * color).trunc() as u16
}

#[cfg(test)]
mod test_conversion_to_eight_bits {
    use super::*;

    #[test]
    fn clamp_min_to_zero() {
        for v in vec![-1e18, -1.0,  -1e-18] {
            assert_eq!(0, to_eight_bits(v), "{}", v);
        }
    }

    #[test]
    fn clamp_max_to_max_u8() {
        for v in vec![1e18, 1.0, 1.0 + 1e-18] {
            assert_eq!(u8::MAX, to_eight_bits(v), "{}", v);
        }
    }

    #[test]
    fn samples_near_borders_resolve_to_appropriate_bucket() {
        let max = u8::MAX as SamplePrecision + 1.0;
        let variance = 1e-6;
        for i in 0..=u8::MAX {
            let v = i as SamplePrecision;
            let lower = (v / max) + variance;
            let upper = ((v + 1.0) / max) - variance;
            assert_eq!(i, to_eight_bits(lower), "lower {}, {}, {}", v, lower, upper);
            assert_eq!(i, to_eight_bits(upper), "upper {}, {}, {}", v, lower, upper);
        }
    }
}

#[cfg(test)]
mod test_conversion_to_sixteen_bits {
    use super::*;

    #[test]
    fn clamp_min_to_zero() {
        for v in vec![-1e18, -1.0,  -1e-18] {
            assert_eq!(0, to_sixteen_bits(v), "{}", v);
        }
    }

    #[test]
    fn clamp_max_to_max_u8() {
        for v in vec![1e18, 1.0, 1.0 + 1e-18] {
            assert_eq!(u16::MAX, to_sixteen_bits(v), "{}", v);
        }
    }

    #[test]
    fn samples_near_borders_resolve_to_appropriate_bucket() {
        let max = u16::MAX as SamplePrecision + 1.0;
        let variance = 1e-9;
        for i in 0..=u16::MAX {
            let v = i as SamplePrecision;
            let lower = (v / max) + variance;
            let upper = ((v + 1.0) / max) - variance;
            assert_eq!(i, to_sixteen_bits(lower), "lower {}, {}, {}", v, lower, upper);
            assert_eq!(i, to_sixteen_bits(upper), "upper {}, {}, {}", v, lower, upper);
        }
    }
}