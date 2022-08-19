/// Compares a `query` string to a list of supported IANA timezones. The return value
/// is a vector of string slices, where the first `count` elements are an (unspecified)
/// ordering of the top `count` autocompletion results.
///
/// # Panic
/// Panics if `count` is greater than or equal to the length of [`TZ_VARIANTS`](chrono_tz::TZ_VARIANTS).
pub fn autocomplete_tz(query: &str, count: usize) -> alloc::vec::Vec<&'static str> {
    let mut names = chrono_tz::TZ_VARIANTS.map(chrono_tz::Tz::name).to_vec();
    names.select_nth_unstable_by(count, move |&a, &b| {
        use strsim::jaro_winkler;
        let first = jaro_winkler(query, a);
        let second = jaro_winkler(query, b);
        second.total_cmp(&first)
    });
    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete_queries() {
        let names = super::autocomplete_tz("Asia/Ma", 3);
        assert_eq!(&names[..3], &["Asia/Macao", "Asia/Macau", "Asia/Manila"]);
    }
}
