#![no_std]

extern crate alloc;

// --- alloc ---
use alloc::{string::String, vec::Vec};
// --- core ---
use core::{char, convert::TryInto};
// // --- crates.io ---
// use thiserror::Error as ThisError;

/// Alias for `Vec<u8>`
pub type Bytes = Vec<u8>;
/// Alias for `String`
pub type Hex = String;
/// The generic main result of crate array-bytes
pub type ArrayBytesResult<T> = Result<T, Error>;

// #[derive(Debug, ThisError)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum Error {
// 	#[error("Invalid length: {}", length)]
// 	InvalidLength { length: usize },
// 	#[error("Invalid char boundary at: {}", index)]
// 	InvalidCharBoundary { index: usize },
// }
/// The main error of crate array-bytes
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error {
	InvalidLength { length: usize },
	InvalidCharBoundary { index: usize },
}

/// `Slice`/`Vec` to `[u8; _]`
///
/// # Examples
///
/// ```
/// assert_eq!([0; 8], array_bytes::dyn2array!(vec![0; 10], 8));
/// ```
#[macro_export]
macro_rules! dyn2array {
	($dyn:expr, $len:expr) => {{
		unsafe { *($dyn.as_ptr() as *const [u8; $len]) }
	}};
}

/// `Hex` to `Bytes`
///
/// Return error while length is a odd number or any byte out of radix
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes("0x4c6f7665204a616e6520466f7265766572").unwrap(),
/// 	*b"Love Jane Forever"
/// );
/// ```
pub fn hex2bytes(hex: impl AsRef<str>) -> ArrayBytesResult<Bytes> {
	let hex = hex.as_ref();

	if hex.len() % 2 != 0 {
		return Err(Error::InvalidLength {
			length: if hex.starts_with("0x") {
				hex.len() - 2
			} else {
				hex.len()
			},
		});
	}

	let mut bytes = Vec::new();

	for i in (if hex.starts_with("0x") { 2 } else { 0 }..hex.len()).step_by(2) {
		for i in i + 1..i + 3 {
			if !hex.is_char_boundary(i) {
				return Err(Error::InvalidCharBoundary { index: i });
			}
		}

		// radix is always 16 which will never fail this; qed
		bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).unwrap());
	}

	Ok(bytes)
}

/// Just like `fn hex2bytes` but without checking
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	array_bytes::hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572"),
/// 	*b"Love Jane Forever"
/// );
/// ```
pub fn hex2bytes_unchecked(hex: impl AsRef<str>) -> Vec<u8> {
	let hex = hex.as_ref();

	(if hex.starts_with("0x") { 2 } else { 0 }..hex.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
		.collect()
}

/// just like `hex2bytes_unchecked` but to a fixed length array
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	array_bytes::hex2array_unchecked("0x4c6f7665204a616e6520466f7265766572"),
/// 	*b"Love Jane Forever"
/// );
/// ```
pub fn hex2array_unchecked<const N: usize>(hex: impl AsRef<str>) -> [u8; N] {
	hex2bytes_unchecked(hex).try_into().unwrap()
}

/// just like `hex2bytes_unchecked` but to a fixed length array
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	array_bytes::hex2array_unchecked!("0x4c6f7665204a616e6520466f7265766572", 17),
/// 	*b"Love Jane Forever"
/// );
/// ```
#[deprecated(since = "1.2.0", note = "use `fn hex2array_unchecked` instead.")]
#[macro_export]
macro_rules! hex2array_unchecked {
	($hex:expr, $len:expr) => {{
		$crate::dyn2array!($crate::hex2bytes_unchecked($hex), $len)
	}};
}

/// `Bytes` to `Hex`
///
/// # Examples
///
/// ```
/// use array_bytes::Hex;
///
/// assert_eq!(
/// 	array_bytes::bytes2hex("0x", b"Love Jane Forever"),
/// 	Hex::from("0x4c6f7665204a616e6520466f7265766572")
/// );
/// ```
pub fn bytes2hex(prefix: impl AsRef<str>, bytes: impl AsRef<[u8]>) -> Hex {
	let prefix = prefix.as_ref();
	let bytes = bytes.as_ref();
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

#[cfg(test)]
mod test {
	// --- array-bytes ---
	use crate::*;

	macro_rules! vec {
		($v:expr; $n:expr) => {{
			let mut v = Vec::new();

			for _ in 0..$n {
				v.push($v);
			}

			v
		}};
	}

	#[test]
	fn dyn2array_should_work() {
		for v in 0u8..16 {
			assert_eq!([v; 8], dyn2array!(vec![v; 10], 8));
		}
	}

	#[test]
	fn hex2bytes_should_work() {
		assert_eq!(hex2bytes("4c6f7665204a616e6520466f7265766572").unwrap(), *b"Love Jane Forever");
		assert_eq!(hex2bytes("0x4c6f7665204a616e6520466f7265766572").unwrap(), *b"Love Jane Forever");

		assert_eq!(
			hex2bytes(Hex::from("我爱你")).unwrap_err(),
			Error::InvalidLength { length: 9 }
		);
		assert_eq!(
			hex2bytes(Hex::from("0x我爱你")).unwrap_err(),
			Error::InvalidLength { length: 9 }
		);

		assert_eq!(
			hex2bytes(Hex::from("我爱你 ")).unwrap_err(),
			Error::InvalidCharBoundary { index: 1 }
		);
		assert_eq!(
			hex2bytes(Hex::from(" 我爱你")).unwrap_err(),
			Error::InvalidCharBoundary { index: 2 }
		);
	}

	#[test]
	fn hex2bytes_unchecked_should_work() {
		assert_eq!(hex2bytes_unchecked("4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
		assert_eq!(
			hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572"),
			*b"Love Jane Forever"
		);
	}

	#[test]
	fn hex2array_unchecked_should_work() {
		assert_eq!(hex2array_unchecked("4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
		assert_eq!(
			hex2array_unchecked("0x4c6f7665204a616e6520466f7265766572"),
			*b"Love Jane Forever"
		);

		assert_eq!(
			hex2array_unchecked!("4c6f7665204a616e6520466f7265766572", 17),
			*b"Love Jane Forever"
		);
		assert_eq!(
			hex2array_unchecked!("0x4c6f7665204a616e6520466f7265766572", 17),
			*b"Love Jane Forever"
		);
	}

	#[test]
	fn bytes2hex_should_work() {
		assert_eq!(
			bytes2hex("", b"Love Jane Forever"),
			Hex::from("4c6f7665204a616e6520466f7265766572")
		);
		assert_eq!(
			bytes2hex("0x", b"Love Jane Forever"),
			Hex::from("0x4c6f7665204a616e6520466f7265766572")
		);
	}
}
