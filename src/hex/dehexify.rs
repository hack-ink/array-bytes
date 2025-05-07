// core
use core::str;
// self
use crate::prelude::*;

static HEX2DIGIT: [Option<u8>; 256] = {
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

/// Dehexify the given hex to `Self`.
///
/// # Examples
/// ```
/// use array_bytes::{Dehexify, Error};
/// use smallvec::SmallVec;
///
/// // Unsigned.
/// assert_eq!(u8::dehexify("34"), Ok(52));
/// assert_eq!(u16::dehexify("208"), Ok(520));
/// assert_eq!(u32::dehexify("0x4f5da2"), Ok(5_201_314));
/// assert_eq!(u64::dehexify("0x4F5DA2"), Ok(5_201_314));
/// assert_eq!(u128::dehexify("4f5da2"), Ok(5_201_314));
/// assert_eq!(usize::dehexify("4F5DA2"), Ok(5_201_314));
/// // Array.
/// assert_eq!(
/// 	<[u8; 17]>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(*b"Love Jane Forever")
/// );
/// // SmallVec.
/// assert_eq!(
/// 	SmallVec::dehexify("0x4c6f7665204a616e6520466f7265766572").unwrap().into_vec(),
/// 	b"Love Jane Forever".to_vec()
/// );
/// assert_eq!(SmallVec::dehexify("我爱你"), Err(Error::InvalidLength));
/// assert_eq!(SmallVec::dehexify("0x我爱你"), Err(Error::InvalidLength));
/// // Vec.
/// assert_eq!(
/// 	<Vec<u8>>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(b"Love Jane Forever".to_vec())
/// );
/// assert_eq!(
/// 	<Vec<u8>>::dehexify("我爱你 "),
/// 	Err(Error::InvalidCharacter { character: 'æ', index: 0 })
/// );
/// assert_eq!(
/// 	<Vec<u8>>::dehexify(" 我爱你"),
/// 	Err(Error::InvalidCharacter { character: ' ', index: 0 })
/// );
/// ```
pub trait Dehexify
where
	Self: Sized,
{
	/// Dehexify `Self` from hex.
	fn dehexify<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>;
}
macro_rules! impl_dehexify_for_unsigned {
	($($t:ty,)+) => {
		$(impl Dehexify for $t {
			fn dehexify<H>(hex: H) -> Result<Self>
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
impl_dehexify_for_unsigned! {
	usize,
	u8,
	u16,
	u32,
	u64,
	u128,
}
impl<const N: usize> Dehexify for [u8; N] {
	fn dehexify<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		dehexify_array(hex)
	}
}
impl Dehexify for SmallVec<[u8; 64]> {
	fn dehexify<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		dehexify_bytes(hex)
	}
}
impl Dehexify for Vec<u8> {
	fn dehexify<H>(hex: H) -> Result<Self>
	where
		H: AsRef<[u8]>,
	{
		dehexify_bytes(hex).map(|sv| sv.into_vec())
	}
}
#[test]
fn dehexify_should_work() {
	// Unsigned.
	assert_eq!(u8::dehexify("34"), Ok(52));
	assert_eq!(u16::dehexify("208"), Ok(520));
	assert_eq!(u32::dehexify("0x4f5da2"), Ok(5_201_314));
	assert_eq!(u64::dehexify("0x4F5DA2"), Ok(5_201_314));
	assert_eq!(u128::dehexify("4f5da2"), Ok(5_201_314));
	assert_eq!(usize::dehexify("4F5DA2"), Ok(5_201_314));
	// Array.
	assert_eq!(
		<[u8; 17]>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
		Ok(*b"Love Jane Forever")
	);
	// SmallVec.
	assert_eq!(
		SmallVec::dehexify("0x4c6f7665204a616e6520466f7265766572").unwrap().into_vec(),
		b"Love Jane Forever".to_vec()
	);
	assert_eq!(SmallVec::dehexify("我爱你"), Err(Error::InvalidLength));
	assert_eq!(SmallVec::dehexify("0x我爱你"), Err(Error::InvalidLength));
	// Vec.
	assert_eq!(
		<Vec<u8>>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
		Ok(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		<Vec<u8>>::dehexify("我爱你 "),
		Err(Error::InvalidCharacter { character: 'æ', index: 0 })
	);
	assert_eq!(
		<Vec<u8>>::dehexify(" 我爱你"),
		Err(Error::InvalidCharacter { character: ' ', index: 0 })
	);
}

/// Dehexify hex into a mutable slice source.
///
/// # Examples
/// ```
/// let mut array = [0; 17];
///
/// assert_eq!(
/// 	array_bytes::dehexify_slice_mut("0x4c6f7665204a616e6520466f7265766572", &mut array),
/// 	Ok(b"Love Jane Forever".as_slice())
/// );
/// assert_eq!(array, *b"Love Jane Forever");
/// ```
pub fn dehexify_slice_mut<H>(hex: H, slice_src: &mut [u8]) -> Result<&[u8]>
where
	H: AsRef<[u8]>,
{
	let hex = strip_0x(hex.as_ref());

	if hex.len() % 2 != 0 {
		Err(Error::InvalidLength)?;
	}

	let expected_len = hex.len() >> 1;

	if expected_len != slice_src.len() {
		Err(Error::MismatchedLength { expect: expected_len })?;
	}

	for (byte, i) in slice_src.iter_mut().zip((0..hex.len()).step_by(2)) {
		*byte = dehexify_ascii((&hex[i], i), (&hex[i + 1], i + 1))?;
	}

	Ok(slice_src)
}
#[test]
fn dehexify_slice_mut_should_work() {
	let mut bytes = [0; 17];
	assert_eq!(
		dehexify_slice_mut("0x4c6f7665204a616e6520466f7265766572", &mut bytes),
		Ok(b"Love Jane Forever".as_slice())
	);
	assert_eq!(bytes, *b"Love Jane Forever");

	let mut bytes = [0; 17];
	assert_eq!(
		dehexify_slice_mut("0x4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
		Ok(b"Love Jane Forever".as_slice())
	);
	assert_eq!(bytes, *b"Love Jane Forever");

	let mut bytes = [0; 17];
	assert_eq!(
		dehexify_slice_mut("4c6f7665204a616e6520466f7265766572", &mut bytes),
		Ok(b"Love Jane Forever".as_slice())
	);
	assert_eq!(bytes, *b"Love Jane Forever");

	let mut bytes = [0; 17];
	assert_eq!(
		dehexify_slice_mut("4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
		Ok(b"Love Jane Forever".as_slice())
	);
	assert_eq!(bytes, *b"Love Jane Forever");

	assert_eq!(dehexify_slice_mut("0", &mut []), Err(Error::InvalidLength));
	assert_eq!(dehexify_slice_mut("0x0", &mut []), Err(Error::InvalidLength));

	assert_eq!(dehexify_slice_mut("00", &mut []), Err(Error::MismatchedLength { expect: 1 }));
	assert_eq!(dehexify_slice_mut("0x0001", &mut []), Err(Error::MismatchedLength { expect: 2 }));

	assert_eq!(
		dehexify_slice_mut("fg", &mut [0]),
		Err(Error::InvalidCharacter { character: 'g', index: 1 })
	);
	assert_eq!(
		dehexify_slice_mut("0xyz", &mut [0]),
		Err(Error::InvalidCharacter { character: 'y', index: 0 })
	);
}

/// Dehexify hex to a fixed length bytes vector then convert it to `T` where `T: From<[u8; N]>`.
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
/// 	array_bytes::dehexify_array_then_into::<_, Ljf, 17>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(Ljf(*b"Love Jane Forever"))
/// );
/// ```
pub fn dehexify_array_then_into<H, T, const N: usize>(hex: H) -> Result<T>
where
	H: AsRef<[u8]>,
	T: From<[u8; N]>,
{
	Ok(dehexify_array(hex)?.into())
}
#[test]
fn dehexify_array_then_into_should_work() {
	assert_eq!(
		dehexify_array_then_into::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
	assert_eq!(
		dehexify_array_then_into::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572"),
		Ok(Ljfn(*b"Love Jane Forever"))
	);

	assert_eq!(
		dehexify_array_then_into::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
	assert_eq!(
		dehexify_array_then_into::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
}

/// Dehexify hex to a bytes vector then convert it to `T` where `T: From<Vec<u8>`.
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
/// 	array_bytes::dehexify_vec_then_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
/// 	Ok(Ljf(b"Love Jane Forever".to_vec()))
/// );
/// ```
pub fn dehexify_vec_then_into<H, T>(hex: H) -> Result<T>
where
	H: AsRef<[u8]>,
	T: From<Vec<u8>>,
{
	Ok(dehexify_bytes(hex.as_ref())?.into_vec().into())
}
#[test]
fn dehexify_vec_then_into_should_work() {
	assert_eq!(
		dehexify_vec_then_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		dehexify_vec_then_into::<_, Ljf>("4c6f7665204a616e6520466f7265766572"),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);

	assert_eq!(
		dehexify_vec_then_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		dehexify_vec_then_into::<_, Ljf>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
}

#[inline(always)]
fn dehexify_array<H, const N: usize>(hex: H) -> Result<[u8; N]>
where
	H: AsRef<[u8]>,
{
	let bytes = dehexify_bytes(hex)?;

	op::slice2array(&bytes)
}

#[inline(always)]
fn dehexify_bytes<H>(hex: H) -> Result<SmallVec<[u8; 64]>>
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
		let high = HEX2DIGIT[hex[i * 2] as usize]
			.ok_or(Error::InvalidCharacter { character: hex[i * 2] as char, index: i * 2 })?;
		let low = HEX2DIGIT[hex[i * 2 + 1] as usize].ok_or(Error::InvalidCharacter {
			character: hex[i * 2 + 1] as char,
			index: i * 2 + 1,
		})?;

		unsafe {
			*bytes_ptr.add(i) = (high << 4) | low;
		}
	}

	Ok(bytes)
}

#[inline(always)]
pub(super) fn strip_0x(hex: &[u8]) -> &[u8] {
	if hex.len() >= 2 && hex[0] == b'0' && hex[1] == b'x' { &hex[2..] } else { hex }
}

#[inline(always)]
fn dehexify_ascii((hex0, hex0_idx): (&u8, usize), (hex1, hex1_idx): (&u8, usize)) -> Result<u8> {
	let ascii = HEX2DIGIT[*hex0 as usize]
		.ok_or(Error::InvalidCharacter { character: *hex0 as _, index: hex0_idx })?
		<< 4 | HEX2DIGIT[*hex1 as usize]
		.ok_or(Error::InvalidCharacter { character: *hex1 as _, index: hex1_idx })?;

	Ok(ascii)
}
