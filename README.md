<div align="center">

# array-bytes
### A Collection of Array/Bytes/Hex Utilities.

[![License GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Checks](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/docsrs/array-bytes)](https://docs.rs/array-bytes)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/array-bytes?color=red&style=plastic)](https://github.com/hack-ink/array-bytes)

</div>

## Abilities
#### `TryFromHex` trait
- Convert hex to num
  - type `AsRef<[u8]> -> isize`
  - type `AsRef<[u8]> -> i8`
  - type `AsRef<[u8]> -> i16`
  - type `AsRef<[u8]> -> i32`
  - type `AsRef<[u8]> -> i64`
  - type `AsRef<[u8]> -> i128`
  - type `AsRef<[u8]> -> usize`
  - type `AsRef<[u8]> -> u8`
  - type `AsRef<[u8]> -> u16`
  - type `AsRef<[u8]> -> u32`
  - type `AsRef<[u8]> -> u64`
  - type `AsRef<[u8]> -> u128`
- Convert hex to array
  - type `AsRef<[u8]> -> [u8; N]`
  - type `AsRef<[u8]> -> SmallVec<[u8; 64]>`
  - type `AsRef<[u8]> -> Vec<u8>`

#### `Hex` trait
- Convert num to hex
  - type `isize -> String`
  - type `i8 -> String`
  - type `i16 -> String`
  - type `i32 -> String`
  - type `i64 -> String`
  - type `i128 -> String`
  - type `usize -> String`
  - type `u8 -> String`
  - type `u16 -> String`
  - type `u32 -> String`
  - type `u64 -> String`
  - type `u128 -> String`
- Convert array to hex
  - type `[u8; N] -> String`
  - type `&[u8; N] -> String`
  - type `&[u8] -> String`
  - type `Vec<u8> -> String`
  - type `&Vec<u8> -> String`

#### `slice` prefixed functions
- Build fixed length `Array` from `Slice`
  - type `&[T] -> [T; N]`
  - type `&[T] -> &[T; N]`
- Transform `Slice` to `G`
  - type `&[T] -> G`
  - e.g. `&[0_u8, ...] -> [u8; 20] -> H160`

#### `prefix` and `suffix` functions
- Prefixes/suffixes the given element to the given slice to make it a fixed-size array of length `N`.

#### `bytes` prefixed functions
- Convert bytes to hex
  - type `AsRef<[u8]> -> String`

#### `hex` prefixed functions
- Convert `HexBytes` to hex
  - type `&[u8] -> &str`
  - e.g. `b"0x..." -> "0x..."`
- Transform hex from `Array`
  - type `&str -> [u8; N]`
- Convert hex to bytes
  - type  `AsRef<[u8]> -> SmallVec<[u8; 64]>`
- Convert hex to `Slice`
  - type `AsRef<[u8]> -> &[u8]`
- Transform hex to `T`
  - type `AsRef<[u8]> -> T`
  - e.g. `"0x..." -> [u8; 20] -> H160`

#### `vec` prefixed functions
- Build fixed length `Array` from `Vec`
  - type `Vec<T> -> [T; N]`
- Transform `Vec` to `G`
  - type `Vec<T> -> G`
  - e.g. `vec![0_u8,  ...] -> [u8; 20] -> H160`

#### Serde support (require feature `serde`)
- `#[serde(deserialize_with = "array_bytes::hex_deserialize_n_into")]`
  - type `S -> T`
  - e.g. `"0x..." -> H160`
- `#[serde(deserialize_with = "array_bytes::de_try_from_hex")]`
  - type `S -> impl TryFromHex`
  - e.g. `"0xA" -> 10_u32`
- `#[serde(serialize_with = "array_bytes::ser_hex/array_bytes::ser_hex_without_prefix")]`
  - type `S -> impl Hex`
  - e.g. `"0x00" -> vec![0_u8]`

## Benchmark results
<div align="right"><sub>Tuesday, January 9th, 2024</sub></div>

```rs
array_bytes::bytes2hex  time:   [11.175 µs 11.198 µs 11.219 µs]
const_hex::encode       time:   [1.2195 µs 1.2381 µs 1.2564 µs]
faster_hex::hex_string  time:   [12.058 µs 12.089 µs 12.123 µs]
faster_hex::hex_encode_fallback
                        time:   [12.055 µs 12.095 µs 12.135 µs]
hex::encode             time:   [73.787 µs 75.290 µs 76.798 µs]
rustc_hex::to_hex       time:   [43.948 µs 44.517 µs 45.504 µs]

array_bytes::hex2bytes  time:   [19.294 µs 19.383 µs 19.500 µs]
array_bytes::hex2bytes_unchecked
                        time:   [19.507 µs 19.666 µs 19.850 µs]
array_bytes::hex2slice  time:   [23.608 µs 24.087 µs 24.598 µs]
array_bytes::hex2slice_unchecked
                        time:   [21.853 µs 22.428 µs 23.048 µs]
const_hex::decode       time:   [13.999 µs 14.018 µs 14.037 µs]
faster_hex::hex_decode  time:   [28.983 µs 29.028 µs 29.075 µs]
faster_hex::hex_decode_unchecked
                        time:   [11.908 µs 11.926 µs 11.947 µs]
faster_hex::hex_decode_fallback
                        time:   [11.909 µs 11.924 µs 11.940 µs]
hex::decode             time:   [96.566 µs 99.398 µs 102.23 µs]
hex::decode_to_slice    time:   [41.424 µs 42.312 µs 43.448 µs]
```

<div align="right">

#### License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>

</div>
