pub mod float;
pub mod sort;

use alloc::vec::Vec;

/// Compares a `query` string to a list of supported IANA timezones. The return value
/// is a vector of string slices, where the first `count` elements are an (unspecified)
/// ordering of the top `count` autocompletion results.
///
/// # Panic
/// Panics if `count` is greater than or equal to the length of [`TZ_VARIANTS`](chrono_tz::TZ_VARIANTS).
pub fn autocomplete_tz(query: &str, count: usize) -> Vec<&'static str> {
    use core::cmp::Reverse;
    use float::TotalDouble;

    let mut names = chrono_tz::TZ_VARIANTS.map(chrono_tz::Tz::name).to_vec();
    let mut cache = hashbrown::HashMap::with_capacity(32);

    sort::top_n_by_key(&mut names, count, |&tz| {
        let score = *cache.entry(tz).or_insert_with(|| textdistance::str::jaro_winkler(tz, query));
        Reverse(TotalDouble(score))
    });

    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete_queries() {
        let names = super::autocomplete_tz("Asia/Ma", 5);
        assert_eq!(&names[..5], &["Asia/Macao", "Asia/Macau", "Asia/Muscat", "Asia/Manila", "Asia/Magadan"]);

        let names = super::autocomplete_tz("Asia/Man", 5);
        assert_eq!(&names[..5], &["Asia/Manila", "Asia/Magadan", "Asia/Amman", "Asia/Macao", "Asia/Macau"]);
    }
}
