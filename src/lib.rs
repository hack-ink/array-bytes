#![deny(clippy::all, missing_docs, unused_crate_dependencies)]
#![allow(clippy::items_after_test_module, clippy::tabs_in_doc_comments)]
#![no_std]

//! A collection of Array/Bytes/Hex utilities with full No-STD compatibility.
//!
//! Completely optimized for blockchain development.
//! Especially the Polkadot-SDK.
//!
//! # Usage
//! Here are a few quick examples of the most commonly used operations: hexifying and dehexifying.
//!
//! However, this crate also offers many other utilities for Array/Bytes/Hex, each with comprehensive documentation and examples. Check them out on [docs.rs](https://docs.rs/array-bytes)!
//!
//! ```rust
//! use array_bytes::{Dehexify, Error, Hexify};
//! use smallvec::SmallVec;
//!
//! // Hexify.
//! // Unsigned.
//! assert_eq!(52_u8.hexify(), "34");
//! assert_eq!(520_u16.hexify_upper(), "208");
//! assert_eq!(5_201_314_u32.hexify_prefixed(), "0x4f5da2");
//! assert_eq!(5_201_314_u64.hexify_prefixed_upper(), "0x4F5DA2");
//! assert_eq!(5_201_314_u128.hexify(), "4f5da2");
//! assert_eq!(5_201_314_usize.hexify_upper(), "4F5DA2");
//! // `[u8; N]`.
//! assert_eq!(*b"Love Jane Forever".hexify(), String::from("4c6f7665204a616e6520466f7265766572"));
//! // `&[u8; N]`.
//! assert_eq!(
//! 	b"Love Jane Forever".hexify_upper(),
//! 	String::from("4C6F7665204A616E6520466F7265766572")
//! );
//! // `&[u8]`.
//! assert_eq!(
//! 	b"Love Jane Forever".as_slice().hexify_prefixed(),
//! 	String::from("0x4c6f7665204a616e6520466f7265766572")
//! );
//! // `Vec<u8>`.
//! assert_eq!(
//! 	b"Love Jane Forever".to_vec().hexify_prefixed_upper(),
//! 	String::from("0x4C6F7665204A616E6520466F7265766572")
//! );
//! // `&Vec<u8>`.
//! assert_eq!(
//! 	(&b"Love Jane Forever".to_vec()).hexify(),
//! 	String::from("4c6f7665204a616e6520466f7265766572")
//! );
//! // Dehexify.
//! // Unsigned.
//! assert_eq!(u8::dehexify("34"), Ok(52));
//! assert_eq!(u16::dehexify("208"), Ok(520));
//! assert_eq!(u32::dehexify("0x4f5da2"), Ok(5_201_314));
//! assert_eq!(u64::dehexify("0x4F5DA2"), Ok(5_201_314));
//! assert_eq!(u128::dehexify("4f5da2"), Ok(5_201_314));
//! assert_eq!(usize::dehexify("4F5DA2"), Ok(5_201_314));
//! // Array.
//! assert_eq!(
//! 	<[u8; 17]>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
//! 	Ok(*b"Love Jane Forever")
//! );
//! // SmallVec.
//! assert_eq!(
//! 	SmallVec::dehexify("0x4c6f7665204a616e6520466f7265766572").unwrap().into_vec(),
//! 	b"Love Jane Forever".to_vec()
//! );
//! assert_eq!(SmallVec::dehexify("我爱你"), Err(Error::InvalidLength));
//! assert_eq!(SmallVec::dehexify("0x我爱你"), Err(Error::InvalidLength));
//! // Vec.
//! assert_eq!(
//! 	<Vec<u8>>::dehexify("0x4c6f7665204a616e6520466f7265766572"),
//! 	Ok(b"Love Jane Forever".to_vec())
//! );
//! assert_eq!(
//! 	<Vec<u8>>::dehexify("我爱你 "),
//! 	Err(Error::InvalidCharacter { character: 'æ', index: 0 })
//! );
//! assert_eq!(
//! 	<Vec<u8>>::dehexify(" 我爱你"),
//! 	Err(Error::InvalidCharacter { character: ' ', index: 0 })
//! );
//! ```

extern crate alloc;

mod hex;
pub use hex::*;

mod op;
pub use op::*;

#[cfg(feature = "serde")] mod serde;
#[cfg(feature = "serde")] pub use serde::*;

mod prelude {
	pub use alloc::{string::String, vec::Vec};

	pub use smallvec::SmallVec;

	pub use crate::{Error, Result};

	pub(crate) use crate::op;

	#[cfg(test)]
	mod test {
		// Suppress `unused_crate_dependencies` error.
		use const_hex as _;
		use criterion as _;
		use faster_hex as _;
		use hex_crate as _;
		use rustc_hex as _;
		use serde as _;
		use serde_json as _;

		// self
		use super::*;

		#[derive(Debug, PartialEq)]
		pub struct Ljf(pub Vec<u8>);
		impl From<Vec<u8>> for Ljf {
			fn from(bytes: Vec<u8>) -> Self {
				Self(bytes)
			}
		}

		#[derive(Debug, PartialEq)]
		pub struct Ljfn(pub [u8; 17]);
		impl From<[u8; 17]> for Ljfn {
			fn from(array: [u8; 17]) -> Self {
				Self(array)
			}
		}
	}
	#[cfg(test)] pub use test::*;
}

#[cfg(feature = "serde")] pub use serde_bytes;

#[allow(missing_docs)]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(missing_docs)]
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
