mod slice;
pub use slice::*;

mod vec;
pub use vec::*;

// core
use core::cmp::Ordering;

/// Prefixes the given element to the given array/slice/vector to make it a fixed-size array of
/// length `N`.
///
/// If the length of the array/slice/vector is already equal to `N`, it returns the
/// array/slice/vector as a fixed-size array.
/// If the length of the array/slice/vector is greater than `N`, it returns the first `N` elements
/// of the array/slice/vector as a fixed-size array.
/// If the length of the array/slice/vector is less than `N`, it creates a new fixed-size array of
/// length `N` and copies the array/slice/vector into it, padding the remaining elements with the
/// given element.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::prefix_with::<_, _, 4>([5, 2, 0, 1], 0), [5, 2, 0, 1]);
/// assert_eq!(array_bytes::prefix_with::<_, _, 4>([5, 2, 0, 1, 3, 1, 4], 0), [5, 2, 0, 1]);
/// assert_eq!(array_bytes::prefix_with::<_, _, 5>([5, 2, 0], 0), [0, 0, 5, 2, 0]);
/// ```
pub fn prefix_with<A, T, const N: usize>(any: A, element: T) -> [T; N]
where
	A: AsRef<[T]>,
	T: Copy,
{
	pad_array(any, element, true)
}
#[test]
fn prefix_with_should_work() {
	assert_eq!(prefix_with::<_, _, 4>([1, 2, 3, 4], 0), [1, 2, 3, 4]);
	assert_eq!(prefix_with::<_, _, 4>([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4]);
	assert_eq!(prefix_with::<_, _, 5>([1, 2, 3], 0), [0, 0, 1, 2, 3]);
}

/// Suffixes the given element to the given array/slice/vector to make it a fixed-size array of
/// length `N`.
///
/// If the length of the array/slice/vector is already equal to `N`, it returns the
/// array/slice/vector as a fixed-size array.
/// If the length of the array/slice/vector is greater than `N`, it returns the first `N` elements
/// of the array/slice/vector as a fixed-size array.
/// If the length of the array/slice/vector is less than `N`, it creates a new fixed-size array of
/// length `N` and copies the array/slice/vector into it, padding the remaining elements with the
/// given element.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::suffix_with::<_, _, 4>([5, 2, 0, 1], 0), [5, 2, 0, 1]);
/// assert_eq!(array_bytes::suffix_with::<_, _, 4>([5, 2, 0, 1, 3, 1, 4], 0), [5, 2, 0, 1]);
/// assert_eq!(array_bytes::suffix_with::<_, _, 5>([5, 2, 0], 0), [5, 2, 0, 0, 0]);
/// ```
pub fn suffix_with<A, T, const N: usize>(any: A, element: T) -> [T; N]
where
	A: AsRef<[T]>,
	T: Copy,
{
	pad_array(any, element, false)
}
#[test]
fn suffix_with_should_work() {
	assert_eq!(suffix_with::<_, _, 4>([1, 2, 3, 4], 0), [1, 2, 3, 4]);
	assert_eq!(suffix_with::<_, _, 4>([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4]);
	assert_eq!(suffix_with::<_, _, 5>([1, 2, 3], 0), [1, 2, 3, 0, 0]);
}

#[inline(always)]
fn pad_array<A, T, const N: usize>(any: A, element: T, pad_start: bool) -> [T; N]
where
	A: AsRef<[T]>,
	T: Copy,
{
	let a = any.as_ref();

	match a.len().cmp(&N) {
		Ordering::Equal => slice2array(a).expect("`a.len() == N`; qed"),
		Ordering::Greater => slice2array(&a[..N]).expect("`a[..N]` has exactly `N` elements; qed"),
		Ordering::Less => {
			let mut padded = [element; N];

			if pad_start {
				padded[N - a.len()..].copy_from_slice(a);
			} else {
				padded[..a.len()].copy_from_slice(a);
			}

			padded
		},
	}
}
