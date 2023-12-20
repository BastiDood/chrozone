use core::cmp::Ordering;

/// Wrapper for [`f64`](f64) which supports total ordering.
pub struct TotalDouble(pub f64);

impl PartialEq for TotalDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
}

impl PartialOrd for TotalDouble {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TotalDouble {}

impl Ord for TotalDouble {
    fn cmp(&self, other: &Self) -> Ordering {
        // SAFETY: Since we use total ordering for the implementation,
        // the `partial_cmp` will always return the `Some` variant.
        self.0.total_cmp(&other.0)
    }
}
