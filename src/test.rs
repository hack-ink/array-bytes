#![allow(clippy::upper_case_acronyms)]

// crates.io
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};
// self
use crate::*;

macro_rules! bytes {
	($v:expr; $n:expr) => {{
		(0..$n).map(|_| $v).collect::<Vec<_>>()
	}};
}

#[derive(Debug, PartialEq)]
struct Ljf(Vec<u8>);
impl From<Vec<u8>> for Ljf {
	fn from(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}
}

#[derive(Debug, PartialEq)]
struct Ljfn([u8; 17]);
impl From<[u8; 17]> for Ljfn {
	fn from(array: [u8; 17]) -> Self {
		Self(array)
	}
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
struct LjfPredefined {
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_0: isize,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_1: i8,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_2: i16,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_3: i32,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_4: i64,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_5: i128,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_6: usize,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_7: u8,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_8: u16,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_9: u32,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_10: u64,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_11: u128,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_12: Vec<u8>,
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_13: [u8; 1],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_14: [u8; 2],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_15: [u8; 3],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_16: [u8; 4],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_17: [u8; 5],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_18: [u8; 6],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_19: [u8; 7],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_20: [u8; 8],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_21: [u8; 9],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_22: [u8; 10],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_23: [u8; 11],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_24: [u8; 12],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_25: [u8; 13],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_26: [u8; 14],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_27: [u8; 15],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_28: [u8; 16],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_29: [u8; 17],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_30: [u8; 18],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_31: [u8; 19],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_32: [u8; 20],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_33: [u8; 21],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_34: [u8; 22],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_35: [u8; 23],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_36: [u8; 24],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_37: [u8; 25],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_38: [u8; 26],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_39: [u8; 27],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_40: [u8; 28],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_41: [u8; 29],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_42: [u8; 30],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_43: [u8; 31],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_44: [u8; 32],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_45: [u8; 33],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_46: [u8; 34],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_47: [u8; 35],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_48: [u8; 36],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_49: [u8; 37],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_50: [u8; 38],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_51: [u8; 39],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_52: [u8; 40],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_53: [u8; 41],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_54: [u8; 42],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_55: [u8; 43],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_56: [u8; 44],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_57: [u8; 45],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_58: [u8; 46],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_59: [u8; 47],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_60: [u8; 48],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_61: [u8; 49],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_62: [u8; 50],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_63: [u8; 51],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_64: [u8; 52],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_65: [u8; 53],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_66: [u8; 54],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_67: [u8; 55],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_68: [u8; 56],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_69: [u8; 57],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_70: [u8; 58],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_71: [u8; 59],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_72: [u8; 60],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_73: [u8; 61],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_74: [u8; 62],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_75: [u8; 63],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_76: [u8; 64],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_77: [u8; 128],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_78: [u8; 256],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_79: [u8; 512],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_80: [u8; 1024],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_81: [u8; 2048],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_82: [u8; 4096],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_83: [u8; 8192],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_84: [u8; 16384],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_85: [u8; 32768],
	#[cfg_attr(
		feature = "serde",
		serde(deserialize_with = "de_try_from_hex", serialize_with = "se_hex")
	)]
	_86: [u8; 65536],
}
impl Default for LjfPredefined {
	fn default() -> Self {
		Self {
			_0: 5,
			_1: 2,
			_2: 0,
			_3: 5_201_314,
			_4: 5_201_314,
			_5: 5_201_314,
			_6: 5,
			_7: 2,
			_8: 0,
			_9: 5_201_314,
			_10: 5_201_314,
			_11: 5_201_314,
			_12: b"Love Jane Forever".to_vec(),
			_13: [5],
			_14: [5, 2],
			_15: [5, 2, 0],
			_16: [5, 2, 0, 1],
			_17: [5, 2, 0, 1, 3],
			_18: [5, 2, 0, 1, 3, 1],
			_19: [5, 2, 0, 1, 3, 1, 4],
			_20: [0; 8],
			_21: [0; 9],
			_22: [0; 10],
			_23: [0; 11],
			_24: [0; 12],
			_25: [0; 13],
			_26: [0; 14],
			_27: [0; 15],
			_28: [0; 16],
			_29: [0; 17],
			_30: [0; 18],
			_31: [0; 19],
			_32: [0; 20],
			_33: [0; 21],
			_34: [0; 22],
			_35: [0; 23],
			_36: [0; 24],
			_37: [0; 25],
			_38: [0; 26],
			_39: [0; 27],
			_40: [0; 28],
			_41: [0; 29],
			_42: [0; 30],
			_43: [0; 31],
			_44: [0; 32],
			_45: [0; 33],
			_46: [0; 34],
			_47: [0; 35],
			_48: [0; 36],
			_49: [0; 37],
			_50: [0; 38],
			_51: [0; 39],
			_52: [0; 40],
			_53: [0; 41],
			_54: [0; 42],
			_55: [0; 43],
			_56: [0; 44],
			_57: [0; 45],
			_58: [0; 46],
			_59: [0; 47],
			_60: [0; 48],
			_61: [0; 49],
			_62: [0; 50],
			_63: [0; 51],
			_64: [0; 52],
			_65: [0; 53],
			_66: [0; 54],
			_67: [0; 55],
			_68: [0; 56],
			_69: [0; 57],
			_70: [0; 58],
			_71: [0; 59],
			_72: [0; 60],
			_73: [0; 61],
			_74: [0; 62],
			_75: [0; 63],
			_76: [0; 64],
			_77: [0; 128],
			_78: [0; 256],
			_79: [0; 512],
			_80: [0; 1024],
			_81: [0; 2048],
			_82: [0; 4096],
			_83: [0; 8192],
			_84: [0; 16384],
			_85: [0; 32768],
			_86: [0; 65536],
		}
	}
}

#[test]
fn try_from_hex_should_work() {
	let ljf = LjfPredefined::default();

	assert_eq!(TryFromHex::try_from_hex("0x5"), Ok(ljf._0));
	assert_eq!(TryFromHex::try_from_hex("5"), Ok(ljf._0));
	assert_eq!(TryFromHex::try_from_hex("0x2"), Ok(ljf._1));
	assert_eq!(TryFromHex::try_from_hex("2"), Ok(ljf._1));
	assert_eq!(TryFromHex::try_from_hex("0x0"), Ok(ljf._2));
	assert_eq!(TryFromHex::try_from_hex("0"), Ok(ljf._2));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._3));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._3));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._4));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._4));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._5));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._5));
	assert_eq!(TryFromHex::try_from_hex("0x5"), Ok(ljf._6));
	assert_eq!(TryFromHex::try_from_hex("5"), Ok(ljf._6));
	assert_eq!(TryFromHex::try_from_hex("0x2"), Ok(ljf._7));
	assert_eq!(TryFromHex::try_from_hex("2"), Ok(ljf._7));
	assert_eq!(TryFromHex::try_from_hex("0x0"), Ok(ljf._8));
	assert_eq!(TryFromHex::try_from_hex("0"), Ok(ljf._8));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._9));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._9));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._10));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._10));
	assert_eq!(TryFromHex::try_from_hex("0x4f5da2"), Ok(ljf._11));
	assert_eq!(TryFromHex::try_from_hex("4f5da2"), Ok(ljf._11));
	assert_eq!(
		TryFromHex::try_from_hex("0x4c6f7665204a616e6520466f7265766572"),
		Ok(ljf._12.clone())
	);
	assert_eq!(TryFromHex::try_from_hex("4c6f7665204a616e6520466f7265766572"), Ok(ljf._12));
	assert_eq!(TryFromHex::try_from_hex("0x05"), Ok(ljf._13));
	assert_eq!(TryFromHex::try_from_hex("05"), Ok(ljf._13));
	assert_eq!(TryFromHex::try_from_hex("0x0502"), Ok(ljf._14));
	assert_eq!(TryFromHex::try_from_hex("0502"), Ok(ljf._14));
	assert_eq!(TryFromHex::try_from_hex("0x050200"), Ok(ljf._15));
	assert_eq!(TryFromHex::try_from_hex("050200"), Ok(ljf._15));
	assert_eq!(TryFromHex::try_from_hex("0x05020001"), Ok(ljf._16));
	assert_eq!(TryFromHex::try_from_hex("05020001"), Ok(ljf._16));
	assert_eq!(TryFromHex::try_from_hex("0x0502000103"), Ok(ljf._17));
	assert_eq!(TryFromHex::try_from_hex("0502000103"), Ok(ljf._17));
	assert_eq!(TryFromHex::try_from_hex("0x050200010301"), Ok(ljf._18));
	assert_eq!(TryFromHex::try_from_hex("050200010301"), Ok(ljf._18));
	assert_eq!(TryFromHex::try_from_hex("0x05020001030104"), Ok(ljf._19));
	assert_eq!(TryFromHex::try_from_hex("05020001030104"), Ok(ljf._19));

	macro_rules! assert_try_from_hex_array {
		($ljf:expr, $(($field:ident, $len:expr)),+,) => {
			$(
				let hex = "0".repeat($len * 2);

				assert_eq!(TryFromHex::try_from_hex(format!("0x{hex}")), Ok($ljf.$field));
				assert_eq!(TryFromHex::try_from_hex(hex), Ok($ljf.$field));
			)+
		};
	}

	assert_try_from_hex_array! {
		ljf,
		(_20, 8),
		(_21, 9),
		(_22, 10),
		(_23, 11),
		(_24, 12),
		(_25, 13),
		(_26, 14),
		(_27, 15),
		(_28, 16),
		(_29, 17),
		(_30, 18),
		(_31, 19),
		(_32, 20),
		(_33, 21),
		(_34, 22),
		(_35, 23),
		(_36, 24),
		(_37, 25),
		(_38, 26),
		(_39, 27),
		(_40, 28),
		(_41, 29),
		(_42, 30),
		(_43, 31),
		(_44, 32),
		(_45, 33),
		(_46, 34),
		(_47, 35),
		(_48, 36),
		(_49, 37),
		(_50, 38),
		(_51, 39),
		(_52, 40),
		(_53, 41),
		(_54, 42),
		(_55, 43),
		(_56, 44),
		(_57, 45),
		(_58, 46),
		(_59, 47),
		(_60, 48),
		(_61, 49),
		(_62, 50),
		(_63, 51),
		(_64, 52),
		(_65, 53),
		(_66, 54),
		(_67, 55),
		(_68, 56),
		(_69, 57),
		(_70, 58),
		(_71, 59),
		(_72, 60),
		(_73, 61),
		(_74, 62),
		(_75, 63),
		(_76, 64),
		(_77, 128),
		(_78, 256),
		(_79, 512),
		(_80, 1024),
		(_81, 2048),
		(_82, 4096),
		(_83, 8192),
		(_84, 16384),
		(_85, 32768),
		(_86, 65536),
	}
}

#[test]
fn hex_should_work() {
	let ljf = LjfPredefined::default();

	assert_eq!(ljf._0.hex("0x"), "0x5");
	assert_eq!(ljf._0.hex(""), "5");
	assert_eq!(ljf._1.hex("0x"), "0x2");
	assert_eq!(ljf._1.hex(""), "2");
	assert_eq!(ljf._2.hex("0x"), "0x0");
	assert_eq!(ljf._2.hex(""), "0");
	assert_eq!(ljf._3.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._3.hex(""), "4f5da2");
	assert_eq!(ljf._4.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._4.hex(""), "4f5da2");
	assert_eq!(ljf._5.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._5.hex(""), "4f5da2");
	assert_eq!(ljf._6.hex("0x"), "0x5");
	assert_eq!(ljf._6.hex(""), "5");
	assert_eq!(ljf._7.hex("0x"), "0x2");
	assert_eq!(ljf._7.hex(""), "2");
	assert_eq!(ljf._8.hex("0x"), "0x0");
	assert_eq!(ljf._8.hex(""), "0");
	assert_eq!(ljf._9.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._9.hex(""), "4f5da2");
	assert_eq!(ljf._10.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._10.hex(""), "4f5da2");
	assert_eq!(ljf._11.hex("0x"), "0x4f5da2");
	assert_eq!(ljf._11.hex(""), "4f5da2");
	assert_eq!((&ljf._12).hex("0x"), "0x4c6f7665204a616e6520466f7265766572");
	assert_eq!(ljf._12.hex(""), "4c6f7665204a616e6520466f7265766572");
	assert_eq!(ljf._13.hex("0x"), "0x05");
	assert_eq!(ljf._13.hex(""), "05");
	assert_eq!(ljf._14.hex("0x"), "0x0502");
	assert_eq!(ljf._14.hex(""), "0502");
	assert_eq!(ljf._15.hex("0x"), "0x050200");
	assert_eq!(ljf._15.hex(""), "050200");
	assert_eq!(ljf._16.hex("0x"), "0x05020001");
	assert_eq!(ljf._16.hex(""), "05020001");
	assert_eq!(ljf._17.hex("0x"), "0x0502000103");
	assert_eq!(ljf._17.hex(""), "0502000103");
	assert_eq!(ljf._18.hex("0x"), "0x050200010301");
	assert_eq!(ljf._18.hex(""), "050200010301");
	assert_eq!(ljf._19.hex("0x"), "0x05020001030104");
	assert_eq!(ljf._19.hex(""), "05020001030104");

	macro_rules! assert_hex_array {
		($ljf:expr, $(($field:ident, $len:expr)),+,) => {
			$(
				let hex = "0".repeat($len * 2);

				assert_eq!($ljf.$field.hex("0x"), format!("0x{hex}"));
				assert_eq!($ljf.$field.hex(""), hex);
			)+
		};
	}

	assert_hex_array! {
		ljf,
		(_20, 8),
		(_21, 9),
		(_22, 10),
		(_23, 11),
		(_24, 12),
		(_25, 13),
		(_26, 14),
		(_27, 15),
		(_28, 16),
		(_29, 17),
		(_30, 18),
		(_31, 19),
		(_32, 20),
		(_33, 21),
		(_34, 22),
		(_35, 23),
		(_36, 24),
		(_37, 25),
		(_38, 26),
		(_39, 27),
		(_40, 28),
		(_41, 29),
		(_42, 30),
		(_43, 31),
		(_44, 32),
		(_45, 33),
		(_46, 34),
		(_47, 35),
		(_48, 36),
		(_49, 37),
		(_50, 38),
		(_51, 39),
		(_52, 40),
		(_53, 41),
		(_54, 42),
		(_55, 43),
		(_56, 44),
		(_57, 45),
		(_58, 46),
		(_59, 47),
		(_60, 48),
		(_61, 49),
		(_62, 50),
		(_63, 51),
		(_64, 52),
		(_65, 53),
		(_66, 54),
		(_67, 55),
		(_68, 56),
		(_69, 57),
		(_70, 58),
		(_71, 59),
		(_72, 60),
		(_73, 61),
		(_74, 62),
		(_75, 63),
		(_76, 64),
		(_77, 128),
		(_78, 256),
		(_79, 512),
		(_80, 1024),
		(_81, 2048),
		(_82, 4096),
		(_83, 8192),
		(_84, 16384),
		(_85, 32768),
		(_86, 65536),
	}
}

#[test]
fn slice2array_should_work() {
	assert_eq!(slice2array::<_, 8>(&[0; 8]), Ok([0; 8]));
}

#[test]
fn prefix_with_should_work() {
	assert_eq!(prefix_with::<_, _, 4>([1, 2, 3, 4], 0), [1, 2, 3, 4]);
	assert_eq!(prefix_with::<_, _, 4>([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4]);
	assert_eq!(prefix_with::<_, _, 5>([1, 2, 3], 0), [0, 0, 1, 2, 3]);
}

#[test]
fn suffix_with_should_work() {
	assert_eq!(suffix_with::<_, _, 4>([1, 2, 3, 4], 0), [1, 2, 3, 4]);
	assert_eq!(suffix_with::<_, _, 4>([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4]);
	assert_eq!(suffix_with::<_, _, 5>([1, 2, 3], 0), [1, 2, 3, 0, 0]);
}

#[test]
fn slice_n_into_should_work() {
	assert_eq!(slice_n_into::<u8, Ljfn, 17>(b"Love Jane Forever"), Ok(Ljfn(*b"Love Jane Forever")));
}

#[test]
fn slice_n_into_unchecked_should_work() {
	assert_eq!(
		slice_n_into_unchecked::<u8, Ljfn, 17>(b"Love Jane Forever"),
		Ljfn(*b"Love Jane Forever")
	);
}

#[test]
fn vec2array_should_work() {
	assert_eq!(vec2array::<_, 8>(bytes![0; 8]), Ok([0; 8]));
}

#[test]
fn vec_n_into_should_work() {
	assert_eq!(
		vec_n_into::<u8, Ljfn, 17>(b"Love Jane Forever".to_vec()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
}

#[test]
fn vec_n_into_unchecked_should_work() {
	assert_eq!(
		vec_n_into_unchecked::<u8, Ljfn, 17>(b"Love Jane Forever".to_vec()),
		Ljfn(*b"Love Jane Forever")
	);
}

#[test]
fn bytes2hex_should_work() {
	assert_eq!(
		bytes2hex("0x", b"Love Jane Forever"),
		String::from("0x4c6f7665204a616e6520466f7265766572")
	);
	assert_eq!(
		bytes2hex("", b"Love Jane Forever"),
		String::from("4c6f7665204a616e6520466f7265766572")
	);
}

#[test]
fn hex_bytes2hex_str_should_work() {
	assert_eq!(
		hex_bytes2hex_str(b"0x4c6f7665204a616e6520466f7265766572"),
		Ok("0x4c6f7665204a616e6520466f7265766572"),
	);
	assert_eq!(
		hex_bytes2hex_str(b"4c6f7665204a616e6520466f7265766572"),
		Ok("4c6f7665204a616e6520466f7265766572"),
	);

	assert_eq!(
		hex_bytes2hex_str(b"4c6f766 5204a616e6520466f7265766572"),
		Err(Error::InvalidCharacter { character: ' ', index: 7 }),
	);
	assert_eq!(
		hex_bytes2hex_str(b"4c6f766520 4a616e6520466f7265766572"),
		Err(Error::InvalidCharacter { character: ' ', index: 10 }),
	);
}

#[test]
fn hex_bytes2hex_str_unchecked_should_work() {
	unsafe {
		assert_eq!(
			hex_bytes2hex_str_unchecked(b"0x4c6f7665204a616e6520466f7265766572"),
			"0x4c6f7665204a616e6520466f7265766572",
		);
		assert_eq!(
			hex_bytes2hex_str_unchecked(b"4c6f7665204a616e6520466f7265766572"),
			"4c6f7665204a616e6520466f7265766572",
		);
	}
}

#[test]
fn hex2array_should_work() {
	assert_eq!(hex2array("0x4c6f7665204a616e6520466f7265766572"), Ok(*b"Love Jane Forever"));
	assert_eq!(
		hex2array("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(*b"Love Jane Forever")
	);
	assert_eq!(hex2array("4c6f7665204a616e6520466f7265766572"), Ok(*b"Love Jane Forever"));
	assert_eq!(
		hex2array("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(*b"Love Jane Forever")
	);
}

#[test]
fn hex2bytes_should_work() {
	assert_eq!(
		hex2bytes("0x4c6f7665204a616e6520466f7265766572"),
		Ok(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		hex2bytes("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(b"Love Jane Forever".to_vec())
	);
	assert_eq!(hex2bytes("4c6f7665204a616e6520466f7265766572"), Ok(b"Love Jane Forever".to_vec()));
	assert_eq!(
		hex2bytes("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(b"Love Jane Forever".to_vec())
	);

	assert_eq!(hex2bytes("我爱你"), Err(Error::InvalidLength));
	assert_eq!(hex2bytes("0x我爱你"), Err(Error::InvalidLength));

	assert_eq!(hex2bytes("我爱你 "), Err(Error::InvalidCharacter { character: 'æ', index: 0 }));
	assert_eq!(hex2bytes(" 我爱你"), Err(Error::InvalidCharacter { character: ' ', index: 0 }));
}

#[test]
fn hex2bytes_unchecked_should_work() {
	assert_eq!(hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
	assert_eq!(
		hex2bytes_unchecked("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		*b"Love Jane Forever"
	);
	assert_eq!(hex2bytes_unchecked("4c6f7665204a616e6520466f7265766572"), *b"Love Jane Forever");
	assert_eq!(
		hex2bytes_unchecked("4c6f7665204a616e6520466f7265766572".as_bytes()),
		*b"Love Jane Forever"
	);
}

#[test]
fn hex2slice_should_work() {
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice("0x4c6f7665204a616e6520466f7265766572", &mut bytes),
			Ok(b"Love Jane Forever".as_slice())
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice("0x4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
			Ok(b"Love Jane Forever".as_slice())
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}

	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice("4c6f7665204a616e6520466f7265766572", &mut bytes),
			Ok(b"Love Jane Forever".as_slice())
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice("4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
			Ok(b"Love Jane Forever".as_slice())
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}

	assert_eq!(hex2slice("0", &mut []), Err(Error::InvalidLength));
	assert_eq!(hex2slice("0x0", &mut []), Err(Error::InvalidLength));

	assert_eq!(hex2slice("00", &mut []), Err(Error::MismatchedLength { expect: 1 }));
	assert_eq!(hex2slice("0x0001", &mut []), Err(Error::MismatchedLength { expect: 2 }));

	assert_eq!(
		hex2slice("fg", &mut [0]),
		Err(Error::InvalidCharacter { character: 'g', index: 1 })
	);
	assert_eq!(
		hex2slice("0xyz", &mut [0]),
		Err(Error::InvalidCharacter { character: 'y', index: 0 })
	);
}

#[test]
fn hex2slice_unchecked_should_work() {
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice_unchecked("0x4c6f7665204a616e6520466f7265766572", &mut bytes),
			b"Love Jane Forever"
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice_unchecked("0x4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
			b"Love Jane Forever"
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}

	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice_unchecked("4c6f7665204a616e6520466f7265766572", &mut bytes),
			b"Love Jane Forever"
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}
	{
		let mut bytes = [0; 17];

		assert_eq!(
			hex2slice_unchecked("4c6f7665204a616e6520466f7265766572".as_bytes(), &mut bytes),
			b"Love Jane Forever"
		);
		assert_eq!(bytes, *b"Love Jane Forever");
	}
}

#[test]
fn hex_into_should_work() {
	assert_eq!(
		hex_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		hex_into::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		hex_into::<_, Ljf>("4c6f7665204a616e6520466f7265766572"),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
	assert_eq!(
		hex_into::<_, Ljf>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljf(b"Love Jane Forever".to_vec()))
	);
}

#[test]
fn hex_n_into_should_work() {
	assert_eq!(
		hex_n_into::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572"),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
	assert_eq!(
		hex_n_into::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
	assert_eq!(
		hex_n_into::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572"),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
	assert_eq!(
		hex_n_into::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ok(Ljfn(*b"Love Jane Forever"))
	);
}

#[test]
fn hex_into_unchecked_should_work() {
	assert_eq!(
		hex_into_unchecked::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572"),
		Ljf(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		hex_into_unchecked::<_, Ljf>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ljf(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		hex_into_unchecked::<_, Ljf>("4c6f7665204a616e6520466f7265766572"),
		Ljf(b"Love Jane Forever".to_vec())
	);
	assert_eq!(
		hex_into_unchecked::<_, Ljf>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ljf(b"Love Jane Forever".to_vec())
	);
}

#[test]
fn hex_n_into_unchecked_should_work() {
	assert_eq!(
		hex_n_into_unchecked::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572"),
		Ljfn(*b"Love Jane Forever")
	);
	assert_eq!(
		hex_n_into_unchecked::<_, Ljfn, 17>("0x4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ljfn(*b"Love Jane Forever")
	);
	assert_eq!(
		hex_n_into_unchecked::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572"),
		Ljfn(*b"Love Jane Forever")
	);
	assert_eq!(
		hex_n_into_unchecked::<_, Ljfn, 17>("4c6f7665204a616e6520466f7265766572".as_bytes()),
		Ljfn(*b"Love Jane Forever")
	);
}

#[cfg(feature = "serde")]
#[test]
fn hex_deserialize_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLjf {
		#[serde(deserialize_with = "hex_deserialize_into")]
		ljf: Ljf,
	}

	assert_eq!(
		serde_json::from_str::<WrappedLjf>(
			r#"{
				"ljf": "0x4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLjf { ljf: Ljf(b"Love Jane Forever".to_vec()) }
	);
	assert_eq!(
		serde_json::from_str::<WrappedLjf>(
			r#"{
				"ljf": "4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLjf { ljf: Ljf(b"Love Jane Forever".to_vec()) }
	);
}

#[cfg(feature = "serde")]
#[test]
fn hex_deserialize_n_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLjf {
		#[serde(deserialize_with = "hex_deserialize_n_into")]
		ljf: Ljfn,
	}

	assert_eq!(
		serde_json::from_str::<WrappedLjf>(
			r#"{
				"ljf": "0x4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLjf { ljf: Ljfn(*b"Love Jane Forever") }
	);
	assert_eq!(
		serde_json::from_str::<WrappedLjf>(
			r#"{
				"ljf": "4c6f7665204a616e6520466f7265766572"
			}"#
		)
		.unwrap(),
		WrappedLjf { ljf: Ljfn(*b"Love Jane Forever") }
	);
}

#[cfg(feature = "serde")]
#[test]
fn de_try_from_hex_should_work() {
	let ljf = LjfPredefined::default();
	let mut json = String::from(
		r#"{
		"_0": "0x5",
		"_1": "2",
		"_2": "0x0",
		"_3": "0x4f5da2",
		"_4": "4f5da2",
		"_5": "0x4f5da2",
		"_6": "5",
		"_7": "0x2",
		"_8": "0",
		"_9": "0x4f5da2",
		"_10": "4f5da2",
		"_11": "0x4f5da2",
		"_12": "0x4c6f7665204a616e6520466f7265766572",
		"_13": "0x05",
		"_14": "0x0502",
		"_15": "0x050200",
		"_16": "0x05020001",
		"_17": "0x0502000103",
		"_18": "0x050200010301",
		"_19": "0x05020001030104","#,
	);

	(20..=76).for_each(|i| {
		json.push_str(&format!(r#""_{i}":"0x{}","#, "0".repeat((i - 12) * 2)));
	});
	json.push_str(&format!(r#""_77":"{}","#, "0".repeat(128 * 2)));
	json.push_str(&format!(r#""_78":"{}","#, "0".repeat(256 * 2)));
	json.push_str(&format!(r#""_79":"{}","#, "0".repeat(512 * 2)));
	json.push_str(&format!(r#""_80":"{}","#, "0".repeat(1024 * 2)));
	json.push_str(&format!(r#""_81":"{}","#, "0".repeat(2048 * 2)));
	json.push_str(&format!(r#""_82":"{}","#, "0".repeat(4096 * 2)));
	json.push_str(&format!(r#""_83":"{}","#, "0".repeat(8192 * 2)));
	json.push_str(&format!(r#""_84":"{}","#, "0".repeat(16384 * 2)));
	json.push_str(&format!(r#""_85":"{}","#, "0".repeat(32768 * 2)));
	json.push_str(&format!(r#""_86":"{}""#, "0".repeat(65536 * 2)));
	json.push('}');

	assert_eq!(ljf, serde_json::from_str::<LjfPredefined>(&json).unwrap());

	let json = json.split(",").map(|l| l.replacen("0x", "", 1)).collect::<Vec<_>>().join(",");

	assert_eq!(ljf, serde_json::from_str::<LjfPredefined>(&json).unwrap());
}

#[cfg(feature = "serde")]
#[test]
fn se_hex_should_work() {
	let ljf = LjfPredefined::default();
	let json = serde_json::to_string(&ljf).unwrap();

	assert_eq!(ljf, serde_json::from_str::<LjfPredefined>(&json).unwrap());
}

#[cfg(feature = "serde")]
#[test]
fn se_hex_without_prefix_should_work() {
	#[derive(Debug, PartialEq, Serialize)]
	struct Ljf {
		#[serde(serialize_with = "se_hex_without_prefix")]
		_0: Vec<u8>,
	}

	let ljf = Ljf { _0: b"Love Jane Forever".to_vec() };

	assert_eq!(
		serde_json::to_string(&ljf).unwrap(),
		r#"{"_0":"4c6f7665204a616e6520466f7265766572"}"#
	);
}

#[test]
fn random_input_should_work() {
	const DATA_1: &[u8] = include_bytes!("lib.rs");
	const DATA_2: &[u8] = include_bytes!("test.rs");

	let data = [DATA_1, DATA_2].concat();

	[8, 16, 32, 64, 128, 256, 512, 1024].into_iter().for_each(|chunks_size| {
		let mut data_pieces = Vec::new();

		data.chunks(chunks_size).enumerate().for_each(|(i, chunk)| {
			data_pieces.push(bytes2hex(if i % 2 == 0 { "0x" } else { "" }, chunk))
		});

		let data_pieces = data_pieces
			.into_iter()
			.map(|piece| match strip_0x(piece.as_bytes()).len() {
				8 => hex2array_unchecked::<_, 4>(&piece).to_vec(),
				32 => hex2array_unchecked::<_, 16>(&piece).to_vec(),
				64 => hex2array_unchecked::<_, 32>(&piece).to_vec(),
				128 => hex2array_unchecked::<_, 64>(&piece).to_vec(),
				256 => hex2array_unchecked::<_, 128>(&piece).to_vec(),
				512 => hex2array_unchecked::<_, 256>(&piece).to_vec(),
				1024 => hex2array_unchecked::<_, 512>(&piece).to_vec(),
				2048 => hex2array_unchecked::<_, 1024>(&piece).to_vec(),
				_ => hex2bytes_unchecked(&piece),
			})
			.collect::<Vec<_>>();

		assert_eq!(data_pieces.concat(), data)
	});
}
