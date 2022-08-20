/// Compares a `query` string to a list of supported IANA timezones. The return value
/// is a vector of string slices, where the first `count` elements are an (unspecified)
/// ordering of the top `count` autocompletion results.
///
/// # Panic
/// Panics if `count` is greater than or equal to the length of [`TZ_VARIANTS`](chrono_tz::TZ_VARIANTS).
pub fn autocomplete_tz(query: &str, count: usize) -> alloc::vec::Vec<&'static str> {
    use core::cmp::Reverse;
    let mut names = chrono_tz::TZ_VARIANTS.map(chrono_tz::Tz::name).to_vec();
    names.select_nth_unstable_by_key(count, move |&target| Reverse(sublime_fuzzy::best_match(query, target)));
    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete_queries() {
        let names = super::autocomplete_tz("Asia/Ma", 5);
        assert_eq!(&names[..5], &["Asia/Manila", "Asia/Macao", "Asia/Macau", "Asia/Magadan", "Asia/Makassar"]);
    }
}
