<div align="center">

# array-bytes
### A collection of Array/Bytes/Hex utilities with full No-STD compatibility.

[![License GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Checks](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/docsrs/array-bytes)](https://docs.rs/array-bytes)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/array-bytes?color=red&style=plastic)](https://github.com/hack-ink/array-bytes)
</div>


## Usage
Here are a few quick examples of the most commonly used operations: hexifying and dehexifying.

However, this crate also offers many other utilities for Array/Bytes/Hex, each with comprehensive documentation and examples. Check them out on [docs.rs](https://docs.rs/array-bytes)!

```rs
use array_bytes::{Dehexify, Hexify};
use smallvec::SmallVec;

// Hexify.
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
assert_eq!(b"Love Jane Forever".hexify_upper(), String::from("4C6F7665204A616E6520466F7265766572"));
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
// Dehexify.
// Unsigned.
assert_eq!(u8::dehexify("34"), Ok(52));
assert_eq!(u16::dehexify("208"), Ok(520));
assert_eq!(u32::dehexify("0x4f5da2"), Ok(5_201_314));
assert_eq!(u64::dehexify("0x4F5DA2"), Ok(5_201_314));
assert_eq!(u128::dehexify("4f5da2"), Ok(5_201_314));
assert_eq!(usize::dehexify("4F5DA2"), Ok(5_201_314));
// Array.
assert_eq!(<[u8; 17]>::dehexify("0x4c6f7665204a616e6520466f7265766572"), Ok(*b"Love Jane Forever"));
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
```

## Benchmark
The following benchmarks were run on a `Apple M4 Max 64GB - macOS 15.2 (24C101)`.

<div align="right"><sub>Fri, Jan 3rd, 2025</sub></div>

```rs
// Hexify.
array_bytes::Hexify::hexify      time: [10.978 µs 10.997 µs 11.021 µs]
const_hex::encode                time: [941.68 ns 946.55 ns 951.44 ns]
faster_hex::hex_string           time: [11.478 µs 11.498 µs 11.519 µs]
faster_hex::hex_encode_fallback  time: [11.546 µs 11.563 µs 11.580 µs]
hex::encode                      time: [85.347 µs 85.524 µs 85.751 µs]
rustc_hex::to_hex                time: [46.267 µs 47.009 µs 47.759 µs]
// Dehexify.
array_bytes::Dehexify::dehexify  time: [19.143 µs 19.156 µs 19.173 µs]
array_bytes::dehexify_slice_mut  time: [20.245 µs 20.274 µs 20.307 µs]
const_hex::decode                time: [13.861 µs 14.276 µs 14.975 µs]
faster_hex::hex_decode           time: [28.499 µs 28.545 µs 28.593 µs]
faster_hex::hex_decode_unchecked time: [11.775 µs 11.799 µs 11.828 µs]
faster_hex::hex_decode_fallback  time: [11.818 µs 11.840 µs 11.862 µs]
hex::decode                      time: [90.870 µs 91.481 µs 92.126 µs]
hex::decode_to_slice             time: [32.272 µs 32.553 µs 32.927 µs]
rustc_hex::from_hex              time: [106.68 µs 107.45 µs 108.31 µs]
```

To run the benchmarks yourself:
```sh
git clone https://github.com/hack-ink/array-bytes
cd array-bytes
cargo bench
```

<div align="right">

## License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>
</div>
