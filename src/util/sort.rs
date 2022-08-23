/// Finds the top `count` elements `items` by the key `compare`. The items
/// outside the range `0..count` are left in an unspecified order.
///
/// # Panic
/// Panics when `count >= items.len()`.
///
/// # Implementation
/// The algorithm repeatedly performs a select operation (based on the
/// partitioning algorithm of quick-sort) until the partition element
/// is at the index `count`. The process of eliminating the search space
/// is reminiscent of bisection in binary search.
///
/// Once the partition element is at the index `count`, we now know two things:
/// * Everything to the left of the partition element compares less.
/// * Everything to the right of the partition element compares greater.
///
/// We thus take the left portion and sort it completely. This is more
/// efficient than sorting everything in the slice as is.
pub fn top_n_by_key<T, F, K>(items: &mut [T], count: usize, mut compare: F)
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    use core::cmp::Ordering::{Equal, Greater, Less};

    let mut haystack = &mut *items;
    let mut index = count;

    loop {
        let (left, _, right) = haystack.select_nth_unstable_by_key(index, &mut compare);
        let curr = left.len();
        (haystack, index) = match curr.cmp(&index) {
            Equal => break,
            Less => (right, index - curr),
            Greater => (left, index),
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn partial_sort_works() {
        let mut items = [2, 1, 4, 5, 0];
        super::top_n_by_key(&mut items, 2, Clone::clone);
        assert_eq!(items, [0, 1, 2, 4, 5]);

        let mut items = [2, 10, 4, 5, 0, 1, 3];
        super::top_n_by_key(&mut items, 3, Clone::clone);
        assert_eq!(items, [0, 1, 2, 3, 4, 5, 10]);

        let mut items = [4, 5, 3, 2, 1];
        super::top_n_by_key(&mut items, 1, Clone::clone);
        assert_eq!(items, [1, 2, 3, 4, 5]);
    }
}
