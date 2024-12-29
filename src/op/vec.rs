// self
use crate::prelude::*;

/// Convert `Vec<T>` to `[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::vec2array::<_, 8>(vec![0; 8]), Ok([0; 8]));
/// ```
pub fn vec2array<T, const N: usize>(vec: Vec<T>) -> Result<[T; N]> {
	vec.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}
#[test]
fn vec2array_should_work() {
	assert_eq!(vec2array::<_, 8>(alloc::vec![0; 8]), Ok([0; 8]));
}

/// Convert `Vec<T>` to `V` where `V: From<[T; N]>`.
///
/// # Examples
///
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
/// 	array_bytes::vec_n_into::<u8, Ljf, 17>(b"Love Jane Forever".to_vec()),
/// 	Ok(Ljf(*b"Love Jane Forever"))
/// );
/// ```
pub fn vec_n_into<T, V, const N: usize>(vec: Vec<T>) -> Result<V>
where
	V: From<[T; N]>,
{
	Ok(vec2array(vec)?.into())
}
#[test]
fn vec_n_into_should_work() {
	assert_eq!(
		vec_n_into::<u8, Ljfn, 17>(b"Love Jane Forever".to_vec()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
}
