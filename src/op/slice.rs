// self
use crate::prelude::*;

/// Convert `&[T]` to `[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::slice2array::<_, 8>(&[5, 2, 0, 1, 3, 1, 4, 0]),
/// 	Ok([5, 2, 0, 1, 3, 1, 4, 0])
/// );
/// ```
#[inline(always)]
pub fn slice2array<T, const N: usize>(slice: &[T]) -> Result<[T; N]>
where
	T: Copy,
{
	slice.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}

#[test]
fn slice2array_should_work() {
	assert_eq!(slice2array::<_, 8>(&[0; 8]), Ok([0; 8]));
}

/// Convert `&[T]` to `&[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::slice2array_ref::<_, 8>(&[5, 2, 0, 1, 3, 1, 4, 0]),
/// 	Ok(&[5, 2, 0, 1, 3, 1, 4, 0])
/// );
/// ```
pub fn slice2array_ref<T, const N: usize>(slice: &[T]) -> Result<&[T; N]>
where
	T: Copy,
{
	slice.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}

/// Convert `&[T]` to `V` where `V: From<[T; N]>`.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct Ljf([u8; 17]);
/// impl From<[u8; 17]> for Ljf {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::slice_n_into::<u8, Ljf, 17>(b"Love Jane Forever"),
/// 	Ok(Ljf(*b"Love Jane Forever"))
/// );
/// ```
pub fn slice_n_into<T, V, const N: usize>(slice: &[T]) -> Result<V>
where
	T: Copy,
	V: From<[T; N]>,
{
	Ok(slice2array(slice)?.into())
}
#[test]
fn slice_n_into_should_work() {
	assert_eq!(slice_n_into::<u8, Ljfn, 17>(b"Love Jane Forever"), Ok(Ljfn(*b"Love Jane Forever")));
}
