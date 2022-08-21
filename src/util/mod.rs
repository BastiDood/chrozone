use alloc::vec::Vec;

pub mod float;

/// Compares a `query` string to a list of supported IANA timezones. The return value
/// is a vector of string slices, where the first `count` elements are an (unspecified)
/// ordering of the top `count` autocompletion results.
///
/// # Panic
/// Panics if `count` is greater than or equal to the length of [`TZ_VARIANTS`](chrono_tz::TZ_VARIANTS).
pub fn autocomplete_tz(query: &str, count: usize) -> Vec<&'static str> {
    let mut names = chrono_tz::TZ_VARIANTS.map(chrono_tz::Tz::name).to_vec();
    let mut cache = hashbrown::HashMap::with_capacity(32);

    let mut haystack = names.as_mut_slice();
    let mut index = count;

    loop {
        // Partition the haystack according to the current index
        let (left, _, right) = haystack.select_nth_unstable_by_key(index, |&tz| {
            let score = *cache.entry(tz).or_insert_with(|| strsim::jaro_winkler(query, tz));
            core::cmp::Reverse(float::TotalDouble(score))
        });

        // Reduce the search space
        use core::cmp::Ordering::{Equal, Greater, Less};
        let curr = left.len();
        (haystack, index) = match curr.cmp(&index) {
            Equal => break,
            Less => (right, index - curr),
            Greater => (left, index),
        };
    }

    // Partially sort the top `count` items
    names[..count].sort_unstable_by_key(|&tz| {
        let score = *cache.entry(tz).or_insert_with(|| strsim::jaro_winkler(query, tz));
        core::cmp::Reverse(float::TotalDouble(score))
    });

    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete_queries() {
        let names = super::autocomplete_tz("Asia/Ma", 5);
        assert_eq!(&names[..5], &["Asia/Macao", "Asia/Macau", "Asia/Manila", "Asia/Magadan", "Asia/Makassar"]);
    }
}
