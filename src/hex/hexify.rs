// core
use core::{mem, str};
// self
use crate::prelude::*;

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Hexify `Self`.
///
/// # Examples
/// ```
/// use array_bytes::Hexify;
///
/// // Unsigned.
/// assert_eq!(52_u8.hexify(), "34");
/// assert_eq!(520_u16.hexify_upper(), "208");
/// assert_eq!(5_201_314_u32.hexify_prefixed(), "0x4f5da2");
/// assert_eq!(5_201_314_u64.hexify_prefixed_upper(), "0x4F5DA2");
/// assert_eq!(5_201_314_u128.hexify(), "4f5da2");
/// assert_eq!(5_201_314_usize.hexify_upper(), "4F5DA2");
/// // `[u8; N]`.
/// assert_eq!(*b"Love Jane Forever".hexify(), String::from("4c6f7665204a616e6520466f7265766572"));
/// // `&[u8; N]`.
/// assert_eq!(
/// 	b"Love Jane Forever".hexify_upper(),
/// 	String::from("4C6F7665204A616E6520466F7265766572")
/// );
/// // `&[u8]`.
/// assert_eq!(
/// 	b"Love Jane Forever".as_slice().hexify_prefixed(),
/// 	String::from("0x4c6f7665204a616e6520466f7265766572")
/// );
/// // `Vec<u8>`.
/// assert_eq!(
/// 	b"Love Jane Forever".to_vec().hexify_prefixed_upper(),
/// 	String::from("0x4C6F7665204A616E6520466F7265766572")
/// );
/// // `&Vec<u8>`.
/// assert_eq!(
/// 	(&b"Love Jane Forever".to_vec()).hexify(),
/// 	String::from("4c6f7665204a616e6520466f7265766572")
/// );
/// ```
pub trait Hexify {
	/// Hexify `Self`.
	fn hexify(&self) -> String;

	/// Hexify `Self` with uppercase.
	fn hexify_upper(&self) -> String;

	/// Hexify `Self` with `0x` prefix.
	fn hexify_prefixed(&self) -> String;

	/// Hexify `Self` with `0x` prefix and uppercase.
	fn hexify_prefixed_upper(&self) -> String;
}
macro_rules! hexify_unsigned {
	($self:expr, $map:expr) => {{
		match $self.highest_set_bit() {
			None => "0".into(),
			Some(high_bit) => {
				let high_nibble = high_bit / 4;
				let nibble_count = high_nibble + 1;
				let mut hex = String::with_capacity(nibble_count as _);

				for nibble in (0..=high_nibble).rev() {
					let shift = nibble * 4;
					let digit = (($self >> shift) & 0xF) as usize;

					hex.push($map[digit] as _);
				}

				hex
			},
		}
	}};
}
macro_rules! hexify_unsigned_prefixed {
	($self:expr, $map:expr) => {{
		match $self.highest_set_bit() {
			None => "0x0".into(),
			Some(high_bit) => {
				let high_nibble = high_bit / 4;
				let nibble_count = high_nibble + 1;
				let mut hex = String::with_capacity(2 + nibble_count as usize);

				hex.push_str("0x");

				for nibble in (0..=high_nibble).rev() {
					let shift = nibble * 4;
					let digit = (($self >> shift) & 0xF) as usize;

					hex.push($map[digit] as _);
				}

				hex
			},
		}
	}};
}
macro_rules! impl_hexify_for_unsigned {
	($($t:ty,)+) => {
		$(
			impl Hexify for $t {
				fn hexify(&self) -> String {
					hexify_unsigned!(self, HEX_CHARS)
				}

				fn hexify_upper(&self) -> String {
					hexify_unsigned!(self, HEX_CHARS_UPPER)
				}

				fn hexify_prefixed(&self) -> String {
					hexify_unsigned_prefixed!(self, HEX_CHARS)
				}

				fn hexify_prefixed_upper(&self) -> String {
					hexify_unsigned_prefixed!(self, HEX_CHARS_UPPER)
				}
			}
		)+
	};
}
impl_hexify_for_unsigned! {
	usize,
	u8,
	u16,
	u32,
	u64,
	u128,
}
macro_rules! hexify {
	($self:expr, $map:expr) => {{
		let cap = $self.len() * 2;
		let mut hex_bytes = <SmallVec<[u8; 128]>>::with_capacity(cap);

		// The capacity is fixed, it's safe to set the length; qed.
		unsafe {
			hex_bytes.set_len(cap);
		}

		let hex_ptr = hex_bytes.as_mut_ptr();

		for (i, &byte) in $self.iter().enumerate() {
			let high = $map[(byte >> 4) as usize];
			let low = $map[(byte & 0x0f) as usize];

			unsafe {
				*hex_ptr.add(i * 2) = high;
				*hex_ptr.add(i * 2 + 1) = low;
			}
		}

		// All the bytes are looked up in the map, it's safe to convert to string; qed.
		unsafe { String::from_utf8_unchecked(hex_bytes.into_vec()) }
	}};
}
macro_rules! hexify_prefixed {
	($self:expr, $map:expr) => {{
		let cap = 2 + $self.len() * 2;
		let mut hex_bytes = <SmallVec<[u8; 128]>>::with_capacity(cap);

		hex_bytes.extend_from_slice(b"0x");

		// The capacity is fixed, it's safe to set the length; qed.
		unsafe {
			hex_bytes.set_len(cap);
		}

		let hex_ptr = unsafe { hex_bytes.as_mut_ptr().add(2) };

		for (i, &byte) in $self.iter().enumerate() {
			let high = $map[(byte >> 4) as usize];
			let low = $map[(byte & 0x0f) as usize];

			unsafe {
				*hex_ptr.add(i * 2) = high;
				*hex_ptr.add(i * 2 + 1) = low;
			}
		}

		// All the bytes are looked up in the map, it's safe to convert to string; qed.
		unsafe { String::from_utf8_unchecked(hex_bytes.into_vec()) }
	}};
}
macro_rules! hexify_bytes_fns {
	() => {
		fn hexify(&self) -> String {
			hexify!(self, HEX_CHARS)
		}

		fn hexify_upper(&self) -> String {
			hexify!(self, HEX_CHARS_UPPER)
		}

		fn hexify_prefixed(&self) -> String {
			hexify_prefixed!(self, HEX_CHARS)
		}

		fn hexify_prefixed_upper(&self) -> String {
			hexify_prefixed!(self, HEX_CHARS_UPPER)
		}
	};
}
impl<const N: usize> Hexify for [u8; N] {
	hexify_bytes_fns! {}
}
impl Hexify for [u8] {
	hexify_bytes_fns! {}
}
impl Hexify for Vec<u8> {
	hexify_bytes_fns! {}
}
#[test]
fn hexify_should_work() {
	// Unsigned.
	assert_eq!(52_u8.hexify(), "34");
	assert_eq!(520_u16.hexify_upper(), "208");
	assert_eq!(5_201_314_u32.hexify_prefixed(), "0x4f5da2");
	assert_eq!(5_201_314_u64.hexify_prefixed_upper(), "0x4F5DA2");
	assert_eq!(5_201_314_u128.hexify(), "4f5da2");
	assert_eq!(5_201_314_usize.hexify_upper(), "4F5DA2");
	// `[u8; N]`.
	assert_eq!(*b"Love Jane Forever".hexify(), String::from("4c6f7665204a616e6520466f7265766572"));
	// `&[u8; N]`.
	assert_eq!(
		b"Love Jane Forever".hexify_upper(),
		String::from("4C6F7665204A616E6520466F7265766572")
	);
	// `&[u8]`.
	assert_eq!(
		b"Love Jane Forever".as_slice().hexify_prefixed(),
		String::from("0x4c6f7665204a616e6520466f7265766572")
	);
	// `Vec<u8>`.
	assert_eq!(
		b"Love Jane Forever".to_vec().hexify_prefixed_upper(),
		String::from("0x4C6F7665204A616E6520466F7265766572")
	);
	// `&Vec<u8>`.
	assert_eq!(
		(&b"Love Jane Forever".to_vec()).hexify(),
		String::from("4c6f7665204a616e6520466f7265766572")
	);
}

trait HighestSetBit {
	fn highest_set_bit(self) -> Option<u32>;
}
macro_rules! impl_highest_set_bit {
	($($t:ty),+ $(,)?) => {
		$(
			impl HighestSetBit for $t {
				fn highest_set_bit(self) -> Option<u32> {
					if self == 0 {
						None
					} else {
						let n_bits = (mem::size_of::<$t>() as u32) * 8;

						Some(n_bits - 1 - self.leading_zeros())
					}
				}
			}
		)+
	}
}
impl_highest_set_bit! {
	u8,
	u16,
	u32,
	u64,
	u128,
	usize
}
#[test]
fn highest_set_bit_should_work() {
	assert_eq!(0_u8.highest_set_bit(), None);
	assert_eq!(1_u16.highest_set_bit(), Some(0));
	assert_eq!(2_u32.highest_set_bit(), Some(1));
	assert_eq!(4_u64.highest_set_bit(), Some(2));
	assert_eq!(8_u128.highest_set_bit(), Some(3));
	assert_eq!(16_usize.highest_set_bit(), Some(4));
}

/// Hexify the bytes which are already in hex.
///
/// This is useful when you are interacting with IO.
///
/// # Examples
/// ```
/// assert_eq!(
/// 	array_bytes::hexify_hex_bytes(b"4c6f7665204a616e6520466f7265766572"),
/// 	Ok("4c6f7665204a616e6520466f7265766572"),
/// );
/// ```
pub fn hexify_hex_bytes(bytes: &[u8]) -> Result<&str> {
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
#[test]
fn hexify_hex_bytes_should_work() {
	assert_eq!(
		hexify_hex_bytes(b"4c6f7665204a616e6520466f7265766572"),
		Ok("4c6f7665204a616e6520466f7265766572"),
	);
	assert_eq!(
		hexify_hex_bytes(b"4C6F7665204A616E6520466F7265766572"),
		Ok("4C6F7665204A616E6520466F7265766572"),
	);
	assert_eq!(
		hexify_hex_bytes(b"0x4c6f7665204a616e6520466f7265766572"),
		Ok("0x4c6f7665204a616e6520466f7265766572"),
	);
	assert_eq!(
		hexify_hex_bytes(b"0x4C6F7665204A616E6520466F7265766572"),
		Ok("0x4C6F7665204A616E6520466F7265766572"),
	);
}
