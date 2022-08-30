#![allow(clippy::upper_case_acronyms)]

// --- array-bytes ---
use crate::*;

macro_rules! bytes {
	($v:expr; $n:expr) => {{
		let mut v = Bytes::new();

		for _ in 0..$n {
			v.push($v);
		}

		v
	}};
}

#[derive(Debug, PartialEq)]
struct LJF(Bytes);
impl From<Bytes> for LJF {
	fn from(bytes: Bytes) -> Self {
		Self(bytes)
	}
}

#[derive(Debug, PartialEq)]
struct LJFN([u8; 17]);
impl From<[u8; 17]> for LJFN {
	fn from(array: [u8; 17]) -> Self {
		Self(array)
	}
}

#[test]
fn slice2array_should_work() {
	assert_eq!(slice2array::<8, _>(&[0; 8]), Ok([0; 8]));
}

#[test]
fn slice_n_into_should_work() {
	assert_eq!(slice_n_into::<17, u8, LJFN>(b"Love Jane Forever"), Ok(LJFN(*b"Love Jane Forever")));
}

#[test]
fn slice_n_into_unchecked_should_work() {
	assert_eq!(
		slice_n_into_unchecked::<17, u8, LJFN>(b"Love Jane Forever"),
		LJFN(*b"Love Jane Forever")
	);
}

#[test]
fn vec2array_should_work() {
	assert_eq!(vec2array::<8, _>(bytes![0; 8]), Ok([0; 8]));
}

#[test]
fn vec_n_into_should_work() {
	assert_eq!(
		vec_n_into::<17, u8, LJFN>(b"Love Jane Forever".to_vec()),
		Ok(LJFN(*b"Love Jane Forever"))
	);
}

#[test]
fn vec_n_into_unchecked_should_work() {
	assert_eq!(
		vec_n_into_unchecked::<17, u8, LJFN>(b"Love Jane Forever".to_vec()),
		LJFN(*b"Love Jane Forever")
	);
}

#[test]
fn bytes2hex_should_work() {
	assert_eq!(
		bytes2hex("0x", b"Love Jane Forever"),
		Hex::from("0x4c6f7665204a616e6520466f7265766572")
	);
	assert_eq!(
		bytes2hex("", b"Love Jane Forever"),
		Hex::from("4c6f7665204a616e6520466f7265766572")
	);
}

#[test]
fn hex2array_should_work() {
	assert_eq!(hex2array("0x4c6f7665204a616e6520466f7265766572"), Ok(*b"Love Jane Forever"));
	assert_eq!(hex2array("4c6f7665204a616e6520466f7265766572"), Ok(*b"Love Jane Forever"));
}

#[test]
fn hex2bytes_should_work() {
	assert_eq!(
		hex2bytes("0x4c6f7665204a616e6520466f7265766572"),
		Ok(b"Love Jane Forever".to_vec())
	);
	assert_eq!(hex2bytes("4c6f7665204a616e6520466f7265766572"), Ok(b"Love Jane Forever".to_vec()));

	assert_eq!(hex2bytes("我爱你").unwrap_err(), Error::InvalidLength { length: 9 });
	assert_eq!(hex2bytes("0x我爱你").unwrap_err(), Error::InvalidLength { length: 9 });

	assert_eq!(hex2bytes("我爱你 ").unwrap_err(), Error::InvalidCharBoundary { index: 1 });
	assert_eq!(hex2bytes(" 我爱你").unwrap_err(), Error::InvalidCharBoundary { index: 2 });
}

#[test]
fn hex2bytes_unchecked_should_work() {
	assert_eq!(hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
	assert_eq!(hex2bytes_unchecked("4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
}

#[test]
fn hex_into_should_work() {
	assert_eq!(
		hex_into::<LJF>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(LJF(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		hex_into::<LJF>("4c6f7665204a616e6520466f7265766572"),
		Ok(LJF(b"Love Jane Forever".to_vec()))
	);
}

#[test]
fn hex_n_into_should_work() {
	assert_eq!(
		hex_n_into::<LJFN, 17>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(LJFN(*b"Love Jane Forever"))
	);
	assert_eq!(
		hex_n_into::<LJFN, 17>("4c6f7665204a616e6520466f7265766572"),
		Ok(LJFN(*b"Love Jane Forever"))
	);
}

#[test]
fn hex_into_unchecked_should_work() {
	assert_eq!(
		hex_into_unchecked::<LJF>("0x4c6f7665204a616e6520466f7265766572"),
		LJF(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		hex_into_unchecked::<LJF>("4c6f7665204a616e6520466f7265766572"),
		LJF(b"Love Jane Forever".to_vec())
	);
}

#[test]
fn hex_n_into_unchecked_should_work() {
	assert_eq!(
		hex_n_into_unchecked::<LJFN, 17>("0x4c6f7665204a616e6520466f7265766572"),
		LJFN(*b"Love Jane Forever")
	);
	assert_eq!(
		hex_n_into_unchecked::<LJFN, 17>("4c6f7665204a616e6520466f7265766572"),
		LJFN(*b"Love Jane Forever")
	);
}

#[cfg(feature = "serde")]
#[test]
fn hex_deserialize_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLJF {
		#[serde(deserialize_with = "hex_deserialize_into")]
		ljf: LJF,
	}

	assert_eq!(
		serde_json::from_str::<WrappedLJF>(
			r#"{
				"ljf": "0x4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLJF { ljf: LJF(b"Love Jane Forever".to_vec()) }
	);
	assert_eq!(
		serde_json::from_str::<WrappedLJF>(
			r#"{
				"ljf": "4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLJF { ljf: LJF(b"Love Jane Forever".to_vec()) }
	);
}

#[cfg(feature = "serde")]
#[test]
fn hex_deserialize_n_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLJF {
		#[serde(deserialize_with = "hex_deserialize_n_into")]
		ljf: LJFN,
	}

	assert_eq!(
		serde_json::from_str::<WrappedLJF>(
			r#"{
				"ljf": "0x4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLJF { ljf: LJFN(*b"Love Jane Forever") }
	);
	assert_eq!(
		serde_json::from_str::<WrappedLJF>(
			r#"{
				"ljf": "4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLJF { ljf: LJFN(*b"Love Jane Forever") }
	);
}

#[cfg(feature = "serde")]
#[test]
fn de_hex2num_should_work() {
	macro_rules! assert_de_hex2num {
		($num_type:ty) => {{
			#[derive(Debug, PartialEq, Deserialize)]
			struct LJFN {
				#[serde(deserialize_with = "de_hex2num")]
				_0: $num_type,
				#[serde(deserialize_with = "de_hex2num")]
				_1: $num_type,
				#[serde(deserialize_with = "de_hex2num")]
				_2: $num_type,
				#[serde(deserialize_with = "de_hex2num")]
				_3: u32,
			}

			assert_eq!(
				serde_json::from_str::<LJFN>(
					r#"{
						"_0": "0x5",
						"_1": "0x2",
						"_2": "0x0",
						"_3": "0x522"
					}"#
				)
				.unwrap(),
				LJFN { _0: 5, _1: 2, _2: 0, _3: 1314 }
			);
			assert_eq!(
				serde_json::from_str::<LJFN>(
					r#"{
						"_0": "5",
						"_1": "2",
						"_2": "0",
						"_3": "522"
					}"#
				)
				.unwrap(),
				LJFN { _0: 5, _1: 2, _2: 0, _3: 1314 }
			);
		}};
	}

	assert_de_hex2num!(isize);
	assert_de_hex2num!(i8);
	assert_de_hex2num!(i16);
	assert_de_hex2num!(i32);
	assert_de_hex2num!(i64);
	assert_de_hex2num!(i128);
	assert_de_hex2num!(usize);
	assert_de_hex2num!(u8);
	assert_de_hex2num!(u16);
	assert_de_hex2num!(u32);
	assert_de_hex2num!(u64);
	assert_de_hex2num!(u128);
}

#[cfg(feature = "serde")]
#[test]
fn de_hex2bytes_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct LJFN {
		#[serde(deserialize_with = "de_hex2bytes")]
		ljf: Bytes,
	}

	assert_eq!(
		serde_json::from_str::<LJFN>(
			r#"{
				"ljf": "0x4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		LJFN { ljf: (*b"Love Jane Forever").to_vec() }
	);
	assert_eq!(
		serde_json::from_str::<LJFN>(
			r#"{
				"ljf": "4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		LJFN { ljf: (*b"Love Jane Forever").to_vec() }
	);
}
