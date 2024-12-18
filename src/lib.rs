#![deny(clippy::all, missing_docs, unused_crate_dependencies)]
#![allow(clippy::tabs_in_doc_comments, clippy::uninit_vec)]
#![no_std]

//! A collection of array/bytes/hex utilities.
//!
//! Completely optimized for blockchain development.
//! Especially the Substrate.

extern crate alloc;

#[cfg(test)] mod test;

// core
use core::{cmp::Ordering, convert::TryInto, str};
// alloc
use alloc::{format, string::String, vec::Vec};
// crates.io
#[cfg(feature = "serde")]
use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};
use smallvec::SmallVec;

/// The main result of array-bytes.
pub type Result<T, E = Error> = core::result::Result<T, E>;

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

static HEX_TO_DIGIT: [Option<u8>; 256] = {
	let mut table = [None; 256];
	let mut i = 0;

	while i <= 9 {
		table[b'0' as usize + i] = Some(i as u8);

		i += 1;
	}

	i = 0;

	while i <= 5 {
		table[b'a' as usize + i] = Some((10 + i) as u8);
		table[b'A' as usize + i] = Some((10 + i) as u8);

		i += 1;
	}

	table
};

/// Try to convert the given hex to a specific type.
///
/// # Examples
/// ```
/// use array_bytes::TryFromHex;
///
/// assert_eq!(u128::try_from_hex("0x5201314"), Ok(85_988_116));
/// ```
pub trait TryFromHex
where
	Self: Sized,
{
	/// Try to convert [`Self`] from hex.
	fn try_from_hex<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>;
}
macro_rules! impl_num_try_from_hex {
	($($t:ty,)+) => {
		$(impl TryFromHex for $t {
			fn try_from_hex<H>(hex: H) -> Result<Self>
			where
				H: AsRef<[u8]>,
			{
				let hex = strip_0x(hex.as_ref());
				let hex = str::from_utf8(hex).map_err(Error::Utf8Error)?;

				Self::from_str_radix(hex, 16).map_err(Error::ParseIntError)
			}
		})+
	};
}
impl_num_try_from_hex! {
	isize,
	i8,
	i16,
	i32,
	i64,
	i128,
	usize,
	u8,
	u16,
	u32,
	u64,
	u128,
}
impl<const N: usize> TryFromHex for [u8; N] {
	fn try_from_hex<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		let bytes = hex2bytes(hex)?;

		slice2array(&bytes)
	}
}
impl TryFromHex for SmallVec<[u8; 64]> {
	fn try_from_hex<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		hex2bytes(hex)
	}
}
impl TryFromHex for Vec<u8> {
	fn try_from_hex<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		hex2bytes(hex).map(|sv| sv.into_vec())
	}
}

/// Convert the given type to hex.
///
/// # Examples
/// ```
/// use array_bytes::Hex;
///
/// assert_eq!(5_201_314_u128.hex("0x"), "0x4f5da2");
/// ```
pub trait Hex {
	/// Convert [`Self`] to hex with the given prefix.
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>;
}
macro_rules! impl_num_hex {
	($($t:ty,)+) => {
		$(
			impl Hex for $t {
				fn hex<P>(self, prefix: P) -> String
				where
					P: AsRef<str>
				{
					let prefix = prefix.as_ref();

					format!("{prefix}{self:x}")
				}
			}
			impl Hex for &$t {
				fn hex<P>(self, prefix: P) -> String
				where
					P: AsRef<str>
				{
					let prefix = prefix.as_ref();

					format!("{prefix}{self:x}")
				}
			}
		)+
	};
}
impl_num_hex! {
	isize,
	i8,
	i16,
	i32,
	i64,
	i128,
	usize,
	u8,
	u16,
	u32,
	u64,
	u128,
}
impl<const N: usize> Hex for [u8; N] {
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>,
	{
		bytes2hex(prefix, self)
	}
}

impl<const N: usize> Hex for &[u8; N] {
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>,
	{
		bytes2hex(prefix, self)
	}
}
impl Hex for &[u8] {
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>,
	{
		bytes2hex(prefix, self)
	}
}
impl Hex for Vec<u8> {
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>,
	{
		bytes2hex(prefix, self)
	}
}
impl Hex for &Vec<u8> {
	fn hex<P>(self, prefix: P) -> String
	where
		P: AsRef<str>,
	{
		bytes2hex(prefix, self)
	}
}

/// The main error of array-bytes.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	/// The length must not be odd.
	InvalidLength,
	/// Found the invalid character at `index`.
	InvalidCharacter {
		/// The invalid character.
		character: char,
		/// The invalid character's index.
		index: usize,
	},
	/// The data can not fit the array/slice length well.
	MismatchedLength {
		/// Expected length.
		expect: usize,
	},
	/// Failed to parse the hex number from hex string.
	Utf8Error(core::str::Utf8Error),
	/// Failed to parse the hex number from hex string.
	ParseIntError(core::num::ParseIntError),
}

/// `&[T]` to `[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::slice2array::<_, 8>(&[5, 2, 0, 1, 3, 1, 4, 0]),
/// 	Ok([5, 2, 0, 1, 3, 1, 4, 0])
/// );
/// ```
pub fn slice2array<T, const N: usize>(slice: &[T]) -> Result<[T; N]>
where
	T: Copy,
{
	slice.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}

/// Just like [`slice2array`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::slice2array_unchecked::<_, 8>(&[5, 2, 0, 1, 3, 1, 4, 0]),
/// 	[5, 2, 0, 1, 3, 1, 4, 0]
/// );
/// ```
pub fn slice2array_unchecked<T, const N: usize>(slice: &[T]) -> [T; N]
where
	T: Copy,
{
	slice2array(slice).unwrap()
}

/// `&[T]` to `&[T; N]`.
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

/// Just like [`slice2array_ref`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::slice2array_ref_unchecked::<_, 8>(&[5, 2, 0, 1, 3, 1, 4, 0]),
/// 	&[5, 2, 0, 1, 3, 1, 4, 0]
/// );
/// ```
pub fn slice2array_ref_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N]
where
	T: Copy,
{
	slice2array_ref(slice).unwrap()
}

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

/// Convert `&[T]` to a type directly.
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

/// Just like [`slice_n_into`] but without the checking.
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
/// 	array_bytes::slice_n_into_unchecked::<u8, Ljf, 17>(b"Love Jane Forever"),
/// 	Ljf(*b"Love Jane Forever")
/// );
/// ```
pub fn slice_n_into_unchecked<T, V, const N: usize>(slice: &[T]) -> V
where
	T: Copy,
	V: From<[T; N]>,
{
	slice2array_unchecked(slice).into()
}

/// [`Vec<T>`] to `[T; N]`.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::vec2array::<_, 8>(vec![0; 8]), Ok([0; 8]));
/// ```
pub fn vec2array<T, const N: usize>(vec: Vec<T>) -> Result<[T; N]> {
	vec.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}

/// Just like [`vec2array`] but without the checking.
///
/// # Examples
/// ```
/// assert_eq!(array_bytes::vec2array_unchecked::<_, 8>(vec![0; 8]), [0; 8]);
/// ```
pub fn vec2array_unchecked<T, const N: usize>(vec: Vec<T>) -> [T; N] {
	vec2array(vec).unwrap()
}

/// Convert [`Vec<T>`] to a type directly.
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

/// Just like [`vec_n_into`] but without the checking.
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
/// 	array_bytes::vec_n_into_unchecked::<u8, Ljf, 17>(b"Love Jane Forever".to_vec()),
/// 	Ljf(*b"Love Jane Forever")
/// );
/// ```
pub fn vec_n_into_unchecked<T, V, const N: usize>(vec: Vec<T>) -> V
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
pub fn hex_bytes2hex_str(bytes: &[u8]) -> Result<&str> {
	for (i, byte) in bytes.iter().enumerate().skip(if bytes.starts_with(b"0x") { 2 } else { 0 }) {
		if !byte.is_ascii_hexdigit() {
			Err(Error::InvalidCharacter { character: *byte as _, index: i })?;
		}
	}

	Ok(
		// Validated in previous step, never fails here; qed.
		unsafe { str::from_utf8_unchecked(bytes) },
	)
}

/// Just like [`hex_bytes2hex_str`] but without the checking.
///
/// # Safety
/// See the [`str::from_utf8_unchecked`].
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex_bytes2hex_str_unchecked(b"0x4c6f7665204a616e6520466f7265766572"),
/// 	"0x4c6f7665204a616e6520466f7265766572",
/// );
/// ```
pub fn hex_bytes2hex_str_unchecked(bytes: &[u8]) -> &str {
	unsafe { str::from_utf8_unchecked(bytes) }
}

/// `AsRef<[u8]>` to [`String`].
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::bytes2hex("0x", b"Love Jane Forever"),
/// 	String::from("0x4c6f7665204a616e6520466f7265766572")
/// );
/// ```
pub fn bytes2hex<P, B>(prefix: P, bytes: B) -> String
where
	P: AsRef<str>,
	B: AsRef<[u8]>,
{
	let prefix = prefix.as_ref();
	let bytes = bytes.as_ref();
	let cap = prefix.len() + bytes.len() * 2;
	let mut hex_bytes = <SmallVec<[u8; 128]>>::with_capacity(cap);

	hex_bytes.extend_from_slice(prefix.as_bytes());

	// The capacity is fixed, it's safe to set the length; qed.
	unsafe {
		hex_bytes.set_len(cap);
	}

	let hex_ptr = unsafe { hex_bytes.as_mut_ptr().add(prefix.len()) };

	for (i, &byte) in bytes.iter().enumerate() {
		let high = HEX_CHARS[(byte >> 4) as usize];
		let low = HEX_CHARS[(byte & 0x0f) as usize];

		unsafe {
			*hex_ptr.add(i * 2) = high;
			*hex_ptr.add(i * 2 + 1) = low;
		}
	}

	// All the bytes are looked up in the `HEX_CHARS`, it's safe to convert to string; qed.
	unsafe { String::from_utf8_unchecked(hex_bytes.into_vec()) }
}

// ? Add `bytes2hex_uppercase`.

/// Just like [`hex2bytes`] but to a fixed length array.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2array("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(*b"Love Jane Forever")
/// );
/// ```
pub fn hex2array<H, const N: usize>(hex: H) -> Result<[u8; N]>
where
	H: AsRef<[u8]>,
{
	vec2array(hex2bytes(hex)?.into_vec())
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
pub fn hex2array_unchecked<H, const N: usize>(hex: H) -> [u8; N]
where
	H: AsRef<[u8]>,
{
	hex2bytes_unchecked(hex).into_vec().try_into().unwrap()
}

/// `AsRef<[u8]>` to [`Vec<u8>`].
///
/// Return error if:
/// - length is odd
/// - encounter invalid hex ascii
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes("0x4c6f7665204a616e6520466f7265766572").unwrap().into_vec(),
/// 	b"Love Jane Forever".to_vec()
/// );
/// ```
pub fn hex2bytes<H>(hex: H) -> Result<SmallVec<[u8; 64]>>
where
	H: AsRef<[u8]>,
{
	let hex = strip_0x(hex.as_ref());

	if hex.len() % 2 != 0 {
		Err(Error::InvalidLength)?;
	}

	let cap = hex.len() / 2;
	let mut bytes = <SmallVec<[u8; 64]>>::with_capacity(cap);

	// The capacity is fixed, it's safe to set the length; qed.
	unsafe {
		bytes.set_len(cap);
	}

	let bytes_ptr = bytes.as_mut_ptr();

	for i in 0..cap {
		let high = HEX_TO_DIGIT[hex[i * 2] as usize]
			.ok_or(Error::InvalidCharacter { character: hex[i * 2] as char, index: i * 2 })?;
		let low = HEX_TO_DIGIT[hex[i * 2 + 1] as usize].ok_or(Error::InvalidCharacter {
			character: hex[i * 2 + 1] as char,
			index: i * 2 + 1,
		})?;

		unsafe {
			*bytes_ptr.add(i) = (high << 4) | low;
		}
	}

	Ok(bytes)
}

/// Just like [`hex2bytes`] but without checking.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572").into_vec(),
/// 	b"Love Jane Forever"
/// );
/// ```
pub fn hex2bytes_unchecked<H>(hex: H) -> SmallVec<[u8; 64]>
where
	H: AsRef<[u8]>,
{
	let hex = strip_0x(hex.as_ref());
	let cap = hex.len() / 2;
	let mut bytes = <SmallVec<[u8; 64]>>::with_capacity(cap);

	// The capacity is fixed, it's safe to set the length; qed.
	unsafe {
		bytes.set_len(cap);
	}

	let bytes_ptr = bytes.as_mut_ptr();

	for i in 0..cap {
		let high = HEX_TO_DIGIT[hex[i * 2] as usize].unwrap();
		let low = HEX_TO_DIGIT[hex[i * 2 + 1] as usize].unwrap();

		unsafe {
			*bytes_ptr.add(i) = (high << 4) | low;
		}
	}

	bytes
}

/// `AsRef<[u8]>` to `&[u8]`.
///
/// This function will modify the given slice's source and return the revised result.
///
/// Return error if:
/// - length is odd
/// - encounter invalid hex ascii
/// - mismatched slice size
///
/// # Examples
/// ```
/// let mut array = [0; 17];
///
/// assert_eq!(
/// 	array_bytes::hex2slice("0x4c6f7665204a616e6520466f7265766572", &mut array),
/// 	Ok(b"Love Jane Forever".as_slice())
/// );
/// assert_eq!(array, *b"Love Jane Forever");
/// ```
pub fn hex2slice<H>(hex: H, slice: &mut [u8]) -> Result<&[u8]>
where
	H: AsRef<[u8]>,
{
	let hex = strip_0x(hex.as_ref());

	if hex.len() % 2 != 0 {
		Err(Error::InvalidLength)?;
	}

	let expected_len = hex.len() >> 1;

	if expected_len != slice.len() {
		Err(Error::MismatchedLength { expect: expected_len })?;
	}

	for (byte, i) in slice.iter_mut().zip((0..hex.len()).step_by(2)) {
		*byte = hex2byte((&hex[i], i), (&hex[i + 1], i + 1))?;
	}

	Ok(slice)
}

/// Just like [`hex2slice`] but without checking.
///
/// # Examples
/// ```
/// let mut array = [0; 17];
///
/// assert_eq!(
/// 	array_bytes::hex2slice_unchecked("0x4c6f7665204a616e6520466f7265766572", &mut array),
/// 	b"Love Jane Forever"
/// );
/// assert_eq!(array, *b"Love Jane Forever");
/// ```
pub fn hex2slice_unchecked<H>(hex: H, slice: &mut [u8]) -> &[u8]
where
	H: AsRef<[u8]>,
{
	let hex = strip_0x(hex.as_ref());

	slice
		.iter_mut()
		.zip((0..hex.len()).step_by(2))
		.for_each(|(byte, i)| *byte = hex2byte_unchecked(&hex[i], &hex[i + 1]));

	slice
}

/// Try to convert `AsRef<[u8]>` to `T` directly, where `T: From<Vec<u8>>`.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct Ljf(Vec<u8>);
/// impl From<Vec<u8>> for Ljf {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(Ljf(b"Love Jane Forever".to_vec()))
/// );
/// ```
pub fn hex_into<H, T>(hex: H) -> Result<T>
where
	H: AsRef<[u8]>,
	T: From<Vec<u8>>,
{
	Ok(hex2bytes(hex.as_ref())?.into_vec().into())
}

/// Just like [`hex_into`] but without the checking.
///
/// # Examples
/// ```
/// #[derive(Debug, PartialEq)]
/// struct Ljf(Vec<u8>);
/// impl From<Vec<u8>> for Ljf {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// assert_eq!(
/// 	array_bytes::hex_into_unchecked::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ljf(b"Love Jane Forever".to_vec())
/// );
/// ```
pub fn hex_into_unchecked<H, T>(hex: H) -> T
where
	H: AsRef<[u8]>,
	T: From<Vec<u8>>,
{
	hex2bytes_unchecked(hex).into_vec().into()
}

/// Try to convert `AsRef<[u8]>` to `T` directly, where `T: From<[u8; N]>`.
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
/// 	array_bytes::hex_n_into::<_, Ljf, 17>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(Ljf(*b"Love Jane Forever"))
/// );
/// ```
pub fn hex_n_into<H, T, const N: usize>(hex: H) -> Result<T>
where
	H: AsRef<[u8]>,
	T: From<[u8; N]>,
{
	Ok(hex2array(hex)?.into())
}

/// Just like [`hex_n_into`] but without the checking.
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
/// 	array_bytes::hex_n_into_unchecked::<_, Ljf, 17>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ljf(*b"Love Jane Forever")
/// );
/// ```
pub fn hex_n_into_unchecked<H, T, const N: usize>(hex: H) -> T
where
	H: AsRef<[u8]>,
	T: From<[u8; N]>,
{
	hex2array_unchecked(hex).into()
}

/// Deserialize hex to `T`, where `T: From<Vec<u8>>`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct Ljf(Vec<u8>);
/// impl From<Vec<u8>> for Ljf {
/// 	fn from(vec: Vec<u8>) -> Self {
/// 		Self(vec)
/// 	}
/// }
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct WrappedLjf {
/// 	#[serde(deserialize_with = "array_bytes::hex_deserialize_into")]
/// 	ljf: Ljf,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<WrappedLjf>(r#"{
/// 		"ljf": "0x4c6f7665204a616e6520466f7265766572"
/// 	}"#).unwrap(),
/// 	WrappedLjf {
/// 		ljf: Ljf(b"Love Jane Forever".to_vec())
/// 	}
/// );
#[cfg(feature = "serde")]
pub fn hex_deserialize_into<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<Vec<u8>>,
{
	let hex_str = <&str>::deserialize(hex)?;

	hex2bytes(hex_str)
		.map(|sv| sv.into_vec().into())
		.map_err(|e| D::Error::custom(format!("{e:?}")))
}

/// Deserialize hex to `T`, where `T: From<[u8; N]>`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct Ljf([u8; 17]);
/// impl From<[u8; 17]> for Ljf {
/// 	fn from(array: [u8; 17]) -> Self {
/// 		Self(array)
/// 	}
/// }
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct WrappedLjf {
/// 	#[serde(deserialize_with = "array_bytes::hex_deserialize_n_into")]
/// 	ljf: Ljf,
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<WrappedLjf>(r#"{
/// 		"ljf": "0x4c6f7665204a616e6520466f7265766572"
/// 	}"#).unwrap(),
/// 	WrappedLjf {
/// 		ljf: Ljf(*b"Love Jane Forever")
/// 	}
/// );
#[cfg(feature = "serde")]
pub fn hex_deserialize_n_into<'de, D, T, const N: usize>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<[u8; N]>,
{
	let hex_str = <&str>::deserialize(hex)?;

	hex2array(hex_str).map(Into::into).map_err(|e| D::Error::custom(format!("{e:?}")))
}

/// Deserialize hex to the pre-defined primitive types.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct Ljf {
/// 	#[serde(deserialize_with = "array_bytes::de_try_from_hex")]
/// 	_0: u8,
/// 	#[serde(deserialize_with = "array_bytes::de_try_from_hex")]
/// 	_1: u16,
/// 	#[serde(deserialize_with = "array_bytes::de_try_from_hex")]
/// 	_2: u32,
/// 	#[serde(deserialize_with = "array_bytes::de_try_from_hex")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::from_str::<Ljf>(
/// 		r#"{
/// 		"_0": "0x5",
/// 		"_1": "0x2",
/// 		"_2": "0x0",
/// 		"_3": "0x01030104"
/// 	}"#
/// 	)
/// 	.unwrap(),
/// 	Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }
/// );
/// ```
#[cfg(feature = "serde")]
pub fn de_try_from_hex<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: TryFromHex,
{
	let hex = <&str>::deserialize(hex)?;

	T::try_from_hex(hex).map_err(|_| D::Error::custom(alloc::format!("invalid hex str `{}`", hex)))
}

/// Serialize the pre-defined primitive types to hex.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hex")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hex")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hex")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hex")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"0x5","_1":"0x2","_2":"0x0","_3":"0x01030104"}"#
/// );
/// ```
#[cfg(feature = "serde")]
pub fn ser_hex<S, T>(hex: T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Hex,
{
	serializer.serialize_str(&hex.hex("0x"))
}
/// Just like [`ser_hex`] but without the prefix.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hex_without_prefix")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hex_without_prefix")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hex_without_prefix")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hex_without_prefix")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"5","_1":"2","_2":"0","_3":"01030104"}"#
/// );
/// ```
#[cfg(feature = "serde")]
pub fn ser_hex_without_prefix<S, T>(hex: T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Hex,
{
	serializer.serialize_str(&hex.hex(""))
}

#[inline(always)]
fn strip_0x(hex: &[u8]) -> &[u8] {
	if let Some(hex) = hex.strip_prefix(b"0x") {
		hex
	} else {
		hex
	}
}

#[inline(always)]
fn hex_ascii2digit(hex_ascii: &u8) -> Option<u8> {
	HEX_TO_DIGIT[*hex_ascii as usize]
}

#[inline(always)]
fn hex2byte(hex_ascii_1: (&u8, usize), hex_ascii_2: (&u8, usize)) -> Result<u8> {
	let byte = hex_ascii2digit(hex_ascii_1.0)
		.ok_or(Error::InvalidCharacter { character: *hex_ascii_1.0 as _, index: hex_ascii_1.1 })?
		<< 4 | hex_ascii2digit(hex_ascii_2.0)
		.ok_or(Error::InvalidCharacter { character: *hex_ascii_2.0 as _, index: hex_ascii_2.1 })?;

	Ok(byte)
}

#[inline(always)]
fn hex2byte_unchecked(hex_ascii_1: &u8, hex_ascii_2: &u8) -> u8 {
	hex_ascii2digit(hex_ascii_1).unwrap() << 4 | hex_ascii2digit(hex_ascii_2).unwrap()
}

#[inline(always)]
fn pad_array<A, T, const N: usize>(any: A, element: T, pad_start: bool) -> [T; N]
where
	A: AsRef<[T]>,
	T: Copy,
{
	let a = any.as_ref();

	match a.len().cmp(&N) {
		// `a.len() == N`; qed.
		Ordering::Equal => slice2array_unchecked(a),
		// `a[..N]` has exactly `N` elements; qed.
		Ordering::Greater => slice2array_unchecked(&a[..N]),
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
