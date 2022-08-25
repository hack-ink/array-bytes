<div align="center">

<!-- Logo -->
<!-- ![array-bytes]() -->

# array-bytes
### Collection of Array/Bytes/Hex Utilities.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/array-bytes?color=red&style=plastic)](https://github.com/hack-ink/array-bytes)

</div>

## Abilities
> Full docs: https://docs.rs/array-bytes/latest/array_bytes

#### `TryFromHex` Trait
- Convert `Hex` to `Num`
	- type `&str -> isize`
	- type `&str -> i8`
	- type `&str -> i16`
	- type `&str -> i32`
	- type `&str -> i64`
	- type `&str -> i128`
	- type `&str -> usize`
	- type `&str -> u8`
	- type `&str -> u16`
	- type `&str -> u32`
	- type `&str -> u64`
	- type `&str -> u128`

#### `bytes` Prefixed Functions
- Convert `Bytes` to `Hex`
  - type `&[u8] -> String`

#### `hex` Prefixed Functions
- Build fixed length `Array` from `Hex`
  - type `&str -> [u8; N]`
- Convert `Hex` to `Bytes`
  - type  `&str -> Vec<u8>`
- Transform `Hex` to `T`
  - type `&str -> T`
  - e.g. `"0x..." -> [u8; 20] -> H160`

#### `slice` Prefixed Functions
- Build fixed length `Array` from `Slice`
  - type `&[T] -> [T; N]`
- Transform `Slice` to `G`
  - type `&[T] -> G`
  - e.g. `&[0_u8, ...] -> [u8; 20] -> H160`

#### `vec` Prefixed Functions
- Build fixed length `Array` from `Vec`
  - type `Vec<T> -> [T; N]`
- Transform `Vec` to `G`
  - type `Vec<T> -> G`
  - e.g. `vec![0_u8,  ...] -> [u8; 20] -> H160`

#### Serde Support (require feature `serde`)
- `#[serde(deserialize_with = "array_bytes::hex_deserialize_into")]`
  - type `S -> T`
  - e.g. `"0x..." -> H160`
- `#[serde(deserialize_with = "array_bytes::de_hex2num")]`
  - type `S -> Num`
  - e.g. `"0xA" -> 10_u32`
- `#[serde(deserialize_with = "array_bytes::de_hex2bytes")]`
  - type `S -> Vec<u8>`
  - e.g. `"0x00" -> vec![0_u8]`
