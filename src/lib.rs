#![no_std]

extern crate alloc;

// --- alloc ---
use alloc::{string::String, vec::Vec};
// --- core ---
use core::char;
// // --- crates.io ---
// use thiserror::Error as ThisError;

pub type ArrayBytesResult<T> = Result<T, Error>;

// #[derive(Debug, ThisError)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum Error {
// 	#[error("Invalid length: {}", length)]
// 	InvalidLength { length: usize },
// 	#[error("Invalid char boundary at: {}", index)]
// 	InvalidCharBoundary { index: usize },
// }
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error {
	InvalidLength { length: usize },
	InvalidCharBoundary { index: usize },
}

/// `Slice`/`Vec` to a fixed length `u8` array
#[macro_export]
macro_rules! array {
	($bytes:expr, $len:expr) => {{
		unsafe { *($bytes.as_ptr() as *const [u8; $len]) }
	}};
}

/// hex to bytes
///
/// error while length is a odd number or any byte out of radix
pub fn hex2bytes(hex: impl AsRef<str>) -> ArrayBytesResult<Vec<u8>> {
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

/// just like `hex2bytes` but without checking
pub fn hex2bytes_unchecked(hex: impl AsRef<str>) -> Vec<u8> {
	let hex = hex.as_ref();

	(if hex.starts_with("0x") { 2 } else { 0 }..hex.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
		.collect()
}

/// just like `hex2bytes_unchecked` but to a fixed length array
#[macro_export]
macro_rules! hex2array_unchecked {
	($hex:expr, $len:expr) => {{
		$crate::array!($crate::hex2bytes_unchecked($hex), $len)
	}};
}

/// bytes to hex
pub fn bytes2hex(prefix: impl AsRef<str>, bytes: impl AsRef<[u8]>) -> String {
	let prefix = prefix.as_ref();
	let bytes = bytes.as_ref();
	let mut hex = String::with_capacity(prefix.len() + bytes.len() * 2);

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
	fn array_should_work() {
		for v in 0u8..16 {
			assert_eq!([v; 8], array!(vec![v; 10], 8));
		}
	}

	#[test]
	fn hex2bytes_should_work() {
		assert_eq!(
			hex2bytes("49204c6f766520596f75").unwrap(),
			b"I Love You".to_vec()
		);
		assert_eq!(
			hex2bytes("0x49204c6f766520596f75").unwrap(),
			b"I Love You".to_vec()
		);

		assert_eq!(
			hex2bytes(String::from("我爱你")).unwrap_err(),
			Error::InvalidLength { length: 9 }
		);
		assert_eq!(
			hex2bytes(String::from("0x我爱你")).unwrap_err(),
			Error::InvalidLength { length: 9 }
		);

		assert_eq!(
			hex2bytes(String::from("我爱你 ")).unwrap_err(),
			Error::InvalidCharBoundary { index: 1 }
		);
		assert_eq!(
			hex2bytes(String::from(" 我爱你")).unwrap_err(),
			Error::InvalidCharBoundary { index: 2 }
		);
	}

	#[test]
	fn hex2bytes_unchecked_should_work() {
		assert_eq!(
			hex2bytes_unchecked("49204c6f766520596f75"),
			b"I Love You".to_vec()
		);
		assert_eq!(
			hex2bytes_unchecked("0x49204c6f766520596f75"),
			b"I Love You".to_vec()
		);
	}

	#[test]
	fn hex2array_unchecked_should_work() {
		assert_eq!(
			hex2array_unchecked!("49204c6f766520596f75", 10),
			*b"I Love You"
		);
		assert_eq!(
			hex2array_unchecked!("0x49204c6f766520596f75", 10),
			*b"I Love You"
		);
	}

	#[test]
	fn bytes2hex_should_work() {
		assert_eq!(
			bytes2hex("", b"I Love You"),
			String::from("49204c6f766520596f75")
		);
		assert_eq!(
			bytes2hex("0x", b"I Love You"),
			String::from("0x49204c6f766520596f75")
		);
	}
}
