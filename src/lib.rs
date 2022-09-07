#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

extern crate alloc;

#[cfg(test)] mod test;

// core
use core::{char, convert::TryInto, mem};
// alloc
use alloc::{string::String, vec::Vec};
// crates.io
#[cfg(feature = "serde")] use serde::{de::Error as DeError, Deserialize, Deserializer};
// use thiserror::Error as ThisError;

/// The generic main result of crate array-bytes.
pub type ArrayBytesResult<T> = Result<T, Error>;

/// Alias for `Vec<u8>`.
pub type Bytes = Vec<u8>;
/// Alias for `String`.
pub type Hex = String;

/// Simple and safe `T`/[`Hex`] conversions that may fail in a controlled way under some
/// circumstances.
pub trait TryFromHex
where
	Self: Sized,
{
	fn try_from_hex(hex: &str) -> ArrayBytesResult<Self>;
}

macro_rules! impl_num_from_hex {
	($t:ty) => {
		impl TryFromHex for $t {
			fn try_from_hex(hex: &str) -> ArrayBytesResult<Self> {
				Self::from_str_radix(&hex[if hex.starts_with("0x") { 2 } else { 0 }..], 16)
					.map_err(Error::ParseIntError)
			}
		}
	};
}
impl_num_from_hex!(isize);
impl_num_from_hex!(i8);
impl_num_from_hex!(i16);
impl_num_from_hex!(i32);
impl_num_from_hex!(i64);
impl_num_from_hex!(i128);
impl_num_from_hex!(usize);
impl_num_from_hex!(u8);
impl_num_from_hex!(u16);
impl_num_from_hex!(u32);
impl_num_from_hex!(u64);
impl_num_from_hex!(u128);

/// The main error of this crate.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	InvalidLength { length: usize },
	InvalidChar { index: usize },
	ParseIntError(core::num::ParseIntError),
}

/// `&[T]` to `[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::slice2array::<8, _>(&[0; 8]), Ok([0; 8]));
/// ```
pub fn slice2array<const N: usize, T>(slice: &[T]) -> ArrayBytesResult<[T; N]>
where
	T: Copy,
{
	slice.try_into().map_err(|_| Error::InvalidLength { length: slice.len() })
}

/// Just like [`slice2array`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::slice2array_unchecked::<8, _>(&[0; 8]), [0; 8]);
/// ```
pub fn slice2array_unchecked<const N: usize, T>(slice: &[T]) -> [T; N]
where
	T: Copy,
{
	slice2array(slice).unwrap()
}

/// Convert `&[T]` to a type directly.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::slice_n_into::<17, u8, LJF>(b"Love Jane Forever"),
/// 	Ok(LJF(*b"Love Jane Forever"))
/// );
/// ```
pub fn slice_n_into<const N: usize, T, V>(slice: &[T]) -> ArrayBytesResult<V>
where
	T: Copy,
	V: From<[T; N]>,
{
	Ok(slice2array(slice)?.into())
}

/// Just like [`slice_n_into`] but without the checking.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::slice_n_into_unchecked::<17, u8, LJF>(b"Love Jane Forever"),
/// 	LJF(*b"Love Jane Forever")
/// );
/// ```
pub fn slice_n_into_unchecked<const N: usize, T, V>(slice: &[T]) -> V
where
	T: Copy,
	V: From<[T; N]>,
{
	slice2array_unchecked(slice).into()
}

/// `Vec<T>` to `[T; M]`.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::vec2array::<8, _>(vec![0; 8]), Ok([0; 8]));
/// ```
pub fn vec2array<const N: usize, T>(vec: Vec<T>) -> ArrayBytesResult<[T; N]> {
	vec.try_into().map_err(|v: Vec<_>| Error::InvalidLength { length: v.len() })
}

/// Just like [`vec2array`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::vec2array_unchecked::<8, _>(vec![0; 8]), [0; 8]);
/// ```
pub fn vec2array_unchecked<const N: usize, T>(vec: Vec<T>) -> [T; N] {
	vec2array(vec).unwrap()
}

/// Convert `Vec<T>` to a type directly.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::vec_n_into::<17, u8, LJF>(b"Love Jane Forever".to_vec()),
/// 	Ok(LJF(*b"Love Jane Forever"))
/// );
/// ```
pub fn vec_n_into<const N: usize, T, V>(vec: Vec<T>) -> ArrayBytesResult<V>
where
	V: From<[T; N]>,
{
	Ok(vec2array(vec)?.into())
}

/// Just like [`vec_n_into`] but without the checking.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::vec_n_into_unchecked::<17, u8, LJF>(b"Love Jane Forever".to_vec()),
/// 	LJF(*b"Love Jane Forever")
/// );
/// ```
pub fn vec_n_into_unchecked<const N: usize, T, V>(vec: Vec<T>) -> V
where
	V: From<[T; N]>,
{
	vec2array_unchecked(vec).into()
}

/// Convert hex bytes to hex string.
///
/// This is useful when you are interacting with the IO.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex_bytes2hex_str(b"0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok("0x4c6f7665204a616e6520466f7265766572"),
/// );
/// ```
pub fn hex_bytes2hex_str(bytes: &[u8]) -> ArrayBytesResult<&str> {
	for (i, byte) in bytes.iter().enumerate().skip(if bytes.starts_with(b"0x") { 2 } else { 0 }) {
		if !is_hex_ascii(byte) {
			Err(Error::InvalidChar { index: i })?;
		}
	}

	Ok(unsafe {
		// Validated in previous step, never fails here; qed.
		#[allow(clippy::transmute_bytes_to_str)]
		mem::transmute(bytes)
	})
}

/// Just like [`hex_bytes2hex_str`] but without the checking.
///
/// # Examples
/// ```
/// unsafe {
/// 	assert_eq!(
/// 		array_bytes::hex_bytes2hex_str_unchecked(b"0x4c6f7665204a616e6520466f7265766572"),
/// 		"0x4c6f7665204a616e6520466f7265766572",
/// 	);
/// }
/// ```
pub unsafe fn hex_bytes2hex_str_unchecked(bytes: &[u8]) -> &str {
	#[allow(clippy::transmute_bytes_to_str)]
	mem::transmute(bytes)
}

/// [`Bytes`] to [`Hex`].
///
/// # Examples
/// ```
/// use array_bytes::Hex;
///
/// assert_eq!(
/// 	array_bytes::bytes2hex("0x", b"Love Jane Forever"),
/// 	Hex::from("0x4c6f7665204a616e6520466f7265766572")
/// );
/// ```
pub fn bytes2hex(prefix: &str, bytes: &[u8]) -> Hex {
	let mut hex = Hex::with_capacity(prefix.len() + bytes.len() * 2);

	for byte in prefix.chars() {
		hex.push(byte);
	}
	for byte in bytes.iter() {
		hex.push(char::from_digit((byte >> 4) as _, 16).unwrap());
		hex.push(char::from_digit((byte & 0xf) as _, 16).unwrap());
	}

	hex
}

/// Just like [`hex2bytes`] but to a fixed length array.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2array("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(*b"Love Jane Forever")
/// );
/// ```
pub fn hex2array<const N: usize>(hex: &str) -> ArrayBytesResult<[u8; N]> {
	vec2array(hex2bytes(hex)?)
}

/// Just like [`hex2array`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2array_unchecked("0x4c6f7665204a616e6520466f7265766572"),
/// 	*b"Love Jane Forever"
/// );
/// ```
pub fn hex2array_unchecked<const N: usize>(hex: &str) -> [u8; N] {
	hex2bytes_unchecked(hex).try_into().unwrap()
}

/// [`Hex`] to [`Bytes`].
///
/// Return error while length is an odd number or any byte out of radix.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(b"Love Jane Forever".to_vec())
/// );
/// ```
pub fn hex2bytes(hex: &str) -> ArrayBytesResult<Bytes> {
	if hex.is_empty() || hex == "0x" {
		return Err(Error::InvalidLength { length: 0 });
	}
	if hex.len() % 2 != 0 {
		return Err(Error::InvalidLength {
			length: if hex.starts_with("0x") { hex.len() - 2 } else { hex.len() },
		});
	}

	let mut bytes = Bytes::new();

	for i in (if hex.starts_with("0x") { 2 } else { 0 }..hex.len()).step_by(2) {
		for i in i..i + 2 {
			if !is_hex_ascii(hex.as_bytes().get(i).unwrap()) {
				return Err(Error::InvalidChar { index: i });
			}
		}

		// radix is always 16 which will never fail this; qed
		bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).unwrap());
	}

	Ok(bytes)
}

/// Just like [`hex2bytes`] but without checking.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572"),
/// 	*b"Love Jane Forever"
/// );
/// ```
pub fn hex2bytes_unchecked(hex: &str) -> Bytes {
	(if hex.starts_with("0x") { 2 } else { 0 }..hex.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
		.collect()
}

/// Try to convert [`Hex`] to `T` directly, where `T: From<Vec<u8>>`.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF(Vec<u8>);
/// impl From<Vec<u8>> for LJF {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_into::<LJF>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(LJF(b"Love Jane Forever".to_vec()))
/// );
/// ```
pub fn hex_into<T>(hex: &str) -> ArrayBytesResult<T>
where
	T: From<Bytes>,
{
	Ok(hex2bytes(hex)?.into())
}

/// Just like [`hex_into`] but without the checking.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF(Vec<u8>);
/// impl From<Vec<u8>> for LJF {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_into_unchecked::<LJF>("0x4c6f7665204a616e6520466f7265766572"),
/// 	LJF(b"Love Jane Forever".to_vec())
/// );
/// ```
pub fn hex_into_unchecked<T>(hex: &str) -> T
where
	T: From<Bytes>,
{
	hex2bytes_unchecked(hex).into()
}

/// Try to convert [`Hex`] to `T` directly, where `T: From<[u8; N]>`.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_n_into::<LJF, 17>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(LJF(*b"Love Jane Forever"))
/// );
/// ```
pub fn hex_n_into<T, const N: usize>(hex: &str) -> ArrayBytesResult<T>
where
	T: From<[u8; N]>,
{
	Ok(hex2array(hex)?.into())
}

/// Just like [`hex_n_into`] but without the checking.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_n_into_unchecked::<LJF, 17>("0x4c6f7665204a616e6520466f7265766572"),
/// 	LJF(*b"Love Jane Forever")
/// );
/// ```
pub fn hex_n_into_unchecked<T, const N: usize>(hex: &str) -> T
where
	T: From<[u8; N]>,
{
	hex2array_unchecked(hex).into()
}

/// Deserialize [`Hex`] to `T`, where `T: From<Vec<u8>>`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct LJF(Vec<u8>);
/// impl From<Vec<u8>> for LJF {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct WrappedLJF {
/// 	#[serde(deserialize_with = "array_bytes::hex_deserialize_into")]
/// 	ljf: LJF,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<WrappedLJF>(r#"{
/// 		"ljf": "0x4c6f7665204a616e6520466f7265766572"
/// 	}"#).unwrap(),
/// 	WrappedLJF {
/// 		ljf: LJF(b"Love Jane Forever".to_vec())
/// 	}
/// );
#[cfg(feature = "serde")]
pub fn hex_deserialize_into<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<Bytes>,
{
	Ok(hex2bytes_unchecked(<&str>::deserialize(hex)?).into())
}

/// Deserialize [`Hex`] to `T`, where `T: From<[u8; N]>`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct LJF([u8; 17]);
/// impl From<[u8; 17]> for LJF {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct WrappedLJF {
/// 	#[serde(deserialize_with = "array_bytes::hex_deserialize_n_into")]
/// 	ljf: LJF,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<WrappedLJF>(r#"{
/// 		"ljf": "0x4c6f7665204a616e6520466f7265766572"
/// 	}"#).unwrap(),
/// 	WrappedLJF {
/// 		ljf: LJF(*b"Love Jane Forever")
/// 	}
/// );
#[cfg(feature = "serde")]
pub fn hex_deserialize_n_into<'de, D, T, const N: usize>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<[u8; N]>,
{
	Ok(hex2array_unchecked(<&str>::deserialize(hex)?).into())
}

/// Deserialize [`Hex`] to any Rust primitive num type.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct LJF {
/// 	#[serde(deserialize_with = "array_bytes::de_hex2num")]
/// 	_0: u8,
/// 	#[serde(deserialize_with = "array_bytes::de_hex2num")]
/// 	_1: u8,
/// 	#[serde(deserialize_with = "array_bytes::de_hex2num")]
/// 	_2: u8,
/// 	#[serde(deserialize_with = "array_bytes::de_hex2num")]
/// 	_3: u32,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<LJF>(
/// 		r#"{
/// 		"_0": "0x5",
/// 		"_1": "0x2",
/// 		"_2": "0x0",
/// 		"_3": "0x522"
/// 	}"#
/// 	)
/// 	.unwrap(),
/// 	LJF { _0: 5, _1: 2, _2: 0, _3: 1314 }
/// );
/// ```
#[cfg(feature = "serde")]
pub fn de_hex2num<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: TryFromHex,
{
	let hex = <&str>::deserialize(hex)?;

	T::try_from_hex(hex).map_err(|_| D::Error::custom(alloc::format!("Invalid hex str `{}`", hex)))
}

/// Deserialize [`Hex`] to [`Bytes`].
///
/// # Examples
/// ```
/// use array_bytes::Bytes;
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct LJF {
/// 	#[serde(deserialize_with = "array_bytes::de_hex2bytes")]
/// 	ljf: Bytes,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<LJF>(
/// 		r#"{
/// 		"ljf": "0x4c6f7665204a616e6520466f7265766572"
/// 	}"#
/// 	)
/// 	.unwrap(),
/// 	LJF { ljf: (*b"Love Jane Forever").to_vec() }
/// );
/// ```
#[cfg(feature = "serde")]
pub fn de_hex2bytes<'de, D>(hex: D) -> Result<Bytes, D::Error>
where
	D: Deserializer<'de>,
{
	let hex = <&str>::deserialize(hex)?;

	hex2bytes(hex).map_err(|_| D::Error::custom(alloc::format!("Invalid hex str `{}`", hex)))
}

fn is_hex_ascii(byte: &u8) -> bool {
	// Convert to lowercase.
	let byte = byte | 0b10_0000;

	matches!(byte, b'0'..=b'9' | b'a'..=b'f')
}
