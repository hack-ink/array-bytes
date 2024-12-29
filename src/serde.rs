// alloc
use alloc::format;
// crates.io
#[cfg(test)] use serde::Serialize;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};
// self
use crate::{prelude::*, Dehexify, Hexify};

/// Serialize `T` to hex.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hexify")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"5","_1":"2","_2":"0","_3":"01030104"}"#
/// );
/// ```
pub fn ser_hexify<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Hexify,
{
	serializer.serialize_str(&value.hexify())
}

/// Serialize `T` to hex with uppercase.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_upper")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_upper")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_upper")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_upper")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"5","_1":"2","_2":"0","_3":"01030104"}"#
/// );
/// ```
pub fn ser_hexify_upper<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Hexify,
{
	serializer.serialize_str(&value.hexify_upper())
}

/// Serialize `T` to hex with `0x` prefix.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"0x5","_1":"0x2","_2":"0x0","_3":"0x01030104"}"#
/// );
/// ```
pub fn ser_hexify_prefixed<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: Hexify,
	S: Serializer,
{
	serializer.serialize_str(&value.hexify_prefixed())
}

/// Serialize `T` to hex with `0x` prefix and uppercase.
///
/// # Examples
/// ```
/// use serde::Serialize;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct Ljf {
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_0: u8,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_1: u16,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_2: u32,
/// 	#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
/// 	_3: [u8; 4],
/// }
///
/// assert_eq!(
/// 	serde_json::to_string::<Ljf>(&Ljf { _0: 5, _1: 2, _2: 0, _3: [1, 3, 1, 4] }).unwrap(),
/// 	r#"{"_0":"0x5","_1":"0x2","_2":"0x0","_3":"0x01030104"}"#
/// );
/// ```
pub fn ser_hexify_prefixed_upper<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	T: Hexify,
{
	serializer.serialize_str(&value.hexify_prefixed())
}

/// Deserialize hex to `T`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct Ljf {
/// 	#[serde(deserialize_with = "array_bytes::de_dehexify")]
/// 	_0: u8,
/// 	#[serde(deserialize_with = "array_bytes::de_dehexify")]
/// 	_1: u16,
/// 	#[serde(deserialize_with = "array_bytes::de_dehexify")]
/// 	_2: u32,
/// 	#[serde(deserialize_with = "array_bytes::de_dehexify")]
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
pub fn de_dehexify<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: Dehexify,
{
	let hex = <&str>::deserialize(hex)?;

	T::dehexify(hex).map_err(|_| D::Error::custom(alloc::format!("invalid hex str `{}`", hex)))
}

/// Deserialize hex to `T` where `T: From<Vec<u8>>`.
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
/// 	#[serde(deserialize_with = "array_bytes::dehexify_vec_then_deserialize_into")]
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
pub fn dehexify_vec_then_deserialize_into<'de, D, T>(hex: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<Vec<u8>>,
{
	let hex = <&str>::deserialize(hex)?;

	<Vec<u8>>::dehexify(hex).map(|sv| sv.into()).map_err(|e| D::Error::custom(format!("{e:?}")))
}
#[test]
fn dehexify_vec_then_deserialize_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLjf {
		#[serde(deserialize_with = "dehexify_vec_then_deserialize_into")]
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

/// Deserialize hex to `T` where `T: From<[u8; N]>`.
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
/// 	#[serde(deserialize_with = "array_bytes::dehexify_array_then_deserialize_into")]
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
pub fn dehexify_array_then_deserialize_into<'de, D, T, const N: usize>(
	hex: D,
) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: From<[u8; N]>,
{
	let hex = <&str>::deserialize(hex)?;

	<[u8; N]>::dehexify(hex).map(Into::into).map_err(|e| D::Error::custom(format!("{e:?}")))
}
#[test]
fn dehexify_array_then_deserialize_into_should_work() {
	#[derive(Debug, PartialEq, Deserialize)]
	struct WrappedLjf {
		#[serde(deserialize_with = "dehexify_array_then_deserialize_into")]
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

#[test]
fn serde_should_work() {
	#[derive(Debug, PartialEq, Deserialize, Serialize)]
	struct LjfPredefined {
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify")]
		_0: u8,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_upper")]
		_1: u16,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_prefixed")]
		_2: u32,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_prefixed_upper")]
		_3: u64,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify")]
		_4: u128,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_upper")]
		_5: usize,
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_prefixed")]
		_6: [u8; 17],
		#[serde(deserialize_with = "de_dehexify", serialize_with = "ser_hexify_prefixed_upper")]
		_7: Vec<u8>,
	}
	impl Default for LjfPredefined {
		fn default() -> Self {
			Self {
				_0: 52,
				_1: 520,
				_2: 5_201_314,
				_3: 5_201_314,
				_4: 5_201_314,
				_5: 5_201_314,
				_6: *b"Love Jane Forever",
				_7: b"Love Jane Forever".to_vec(),
			}
		}
	}

	let ljf = LjfPredefined::default();
	let result = serde_json::to_string(&ljf);
	assert!(result.is_ok());

	let json = result.unwrap();
	assert_eq!(
		json,
		r#"{"_0":"34","_1":"208","_2":"0x4f5da2","_3":"0x4f5da2","_4":"4f5da2","_5":"4F5DA2","_6":"0x4c6f7665204a616e6520466f7265766572","_7":"0x4c6f7665204a616e6520466f7265766572"}"#
	);

	let result = serde_json::from_str::<LjfPredefined>(&json);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), ljf);
}
