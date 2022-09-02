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
- `#[serde(deserialize_with = "array_bytes::hex_deserialize_n_into")]`
  - type `S -> T`
  - e.g. `"0x..." -> H160`
- `#[serde(deserialize_with = "array_bytes::de_hex2num")]`
  - type `S -> Num`
  - e.g. `"0xA" -> 10_u32`
- `#[serde(deserialize_with = "array_bytes::de_hex2bytes")]`
  - type `S -> Vec<u8>`
  - e.g. `"0x00" -> vec![0_u8]`

## Benchmark Results (09/02/2022)
```
array_bytes::bytes2hex  time:   [33.899 µs 34.282 µs 34.708 µs]
                        change: [-0.3915% +1.1844% +2.5863%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) high mild
  14 (14.00%) high severe

hex::encode             time:   [51.175 µs 51.264 µs 51.363 µs]
                        change: [+2.0994% +2.4153% +2.7523%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

rustc_hex::to_hex       time:   [29.986 µs 30.047 µs 30.109 µs]
                        change: [+2.8693% +3.1627% +3.4581%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

faster_hex::hex_string  time:   [8.9542 µs 8.9779 µs 9.0060 µs]
                        change: [+4.2246% +5.4001% +7.1149%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

faster_hex::hex_encode_fallback
                        time:   [7.0233 µs 7.0515 µs 7.0826 µs]
                        change: [+3.5567% +4.0274% +4.4658%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

array_bytes::hex2bytes  time:   [100.96 µs 101.50 µs 102.36 µs]
                        change: [+2.0328% +2.4346% +2.9052%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

array_bytes::hex2bytes_unchecked
                        time:   [92.686 µs 92.884 µs 93.105 µs]
                        change: [+2.5338% +2.9441% +3.3840%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

hex::decode             time:   [101.49 µs 101.79 µs 102.07 µs]
                        change: [+2.1031% +2.5239% +2.9717%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

rustc_hex::from_hex     time:   [66.393 µs 66.582 µs 66.771 µs]
                        change: [+2.4947% +2.8302% +3.1924%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

faster_hex::hex_decode  time:   [14.691 µs 14.721 µs 14.755 µs]
                        change: [+2.2650% +2.5324% +2.8132%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

faster_hex::hex_decode_unchecked
                        time:   [6.2365 µs 6.2541 µs 6.2712 µs]
                        change: [+2.8963% +3.1983% +3.5016%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

faster_hex::hex_decode_fallback
                        time:   [6.2442 µs 6.2607 µs 6.2774 µs]
                        change: [+2.4825% +2.8756% +3.2449%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high sever
```
