use geometry::vec3::*;
use std::cmp::Ordering;

/// Floating point numbers can't be sorted in Rust since they don't implement
/// Ord. FloatCmp may be used instead. Values are assumed to be non-NAN and
/// non-INFINITY.
#[derive(Clone, Copy)]
pub struct FloatCmp(pub Dimension);

impl Ord for FloatCmp {
    fn cmp(&self, other: &FloatCmp) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for FloatCmp {
    fn partial_cmp(&self, other: &FloatCmp) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for FloatCmp {}

impl PartialEq for FloatCmp {
    fn eq(&self, other: &FloatCmp) -> bool {
        self == other
    }
}
