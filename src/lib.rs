#![deny(clippy::all, missing_docs, unused_crate_dependencies)]
#![allow(clippy::items_after_test_module, clippy::tabs_in_doc_comments)]
#![no_std]

//! A collection of Array/Bytes/Hex utilities with full No-STD compatibility.
//!
//! Completely optimized for blockchain development.
//! Especially the Polkadot-SDK.

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
