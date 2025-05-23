pub mod float;
pub mod sort;

/// Compares a `query` string to a list of supported IANA timezones. The return value
/// is a vector of string slices, where the first `count` elements are an (unspecified)
/// ordering of the top `count` autocompletion results.
///
/// # Panic
/// Panics if `count` is greater than or equal to the number of timezones in the system database.
pub fn autocomplete_tz(query: &str, count: usize) -> Box<[Box<str>]> {
    use core::cmp::Reverse;
    use float::TotalDouble;
    use std::sync::LazyLock;

    static CACHED_TIMEZONES: LazyLock<Box<[Box<str>]>> = LazyLock::new(|| {
        jiff::tz::db()
            .available()
            .map(|tz| tz.as_str().into())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    });

    let mut names = CACHED_TIMEZONES.clone();
    let mut cache = hashbrown::HashMap::with_capacity(32);

    sort::top_n_by_key(&mut names, count, |tz| {
        if !cache.contains_key(tz) {
            let score = textdistance::str::jaro_winkler(tz, query);
            let tz = tz.clone();
            unsafe { cache.insert_unique_unchecked(tz, score) };
        }
        Reverse(TotalDouble(cache[tz]))
    });

    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete_queries() {
        let names = super::autocomplete_tz("Asia/Ma", 5);
        assert_eq!(
            &names[..5],
            &[
                "Asia/Macao".into(),
                "Asia/Macau".into(),
                "Asia/Muscat".into(),
                "Asia/Manila".into(),
                "Asia/Magadan".into()
            ]
        );

        let names = super::autocomplete_tz("Asia/Man", 5);
        assert_eq!(
            &names[..5],
            &[
                "Asia/Manila".into(),
                "Asia/Magadan".into(),
                "Asia/Amman".into(),
                "Asia/Macao".into(),
                "Asia/Macau".into()
            ]
        );
    }
}
