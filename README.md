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
  - type `AsRef<[u8]> -> [u8; N]`, `N = { [1, 64], 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536 }`
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
  - type `Vec<u8> -> String`
  - type `[u8; N] -> String`, `N = { [1, 64], 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536 }`
  - type `&[u8] -> String`

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
  - type  `AsRef<[u8]> -> Vec<u8>`
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
- `#[serde(serialize_with = "array_bytes::se_hex")]`
  - type `S -> impl Hex`
  - e.g. `"0x00" -> vec![0_u8]`

## Benchmark results
<div align="right"><sub>Tuesday, January 9th, 2024</sub></div>

```rs
array_bytes::bytes2hex  time:   [26.426 µs 26.473 µs 26.518 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

const_hex::encode       time:   [994.78 ns 1.0084 µs 1.0232 µs]

faster_hex::hex_string  time:   [11.728 µs 11.769 µs 11.815 µs]

faster_hex::hex_encode_fallback
                        time:   [11.704 µs 11.737 µs 11.773 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

hex::encode             time:   [86.105 µs 86.250 µs 86.433 µs]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe

rustc_hex::to_hex       time:   [44.486 µs 45.538 µs 46.723 µs]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  13 (13.00%) high severe

array_bytes::hex2bytes  time:   [43.576 µs 44.529 µs 45.404 µs]
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) low mild
  4 (4.00%) high mild

array_bytes::hex2bytes_unchecked
                        time:   [64.190 µs 65.311 µs 66.359 µs]

array_bytes::hex2slice  time:   [45.484 µs 46.988 µs 48.736 µs]
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe

array_bytes::hex2slice_unchecked
                        time:   [62.339 µs 63.317 µs 64.279 µs]
                        Performance has regressed.

const_hex::decode       time:   [13.601 µs 13.629 µs 13.665 µs]
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe

faster_hex::hex_decode  time:   [28.015 µs 28.061 µs 28.110 µs]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

faster_hex::hex_decode_unchecked
                        time:   [11.782 µs 11.797 µs 11.812 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild

faster_hex::hex_decode_fallback
                        time:   [11.748 µs 11.767 µs 11.785 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild

hex::decode             time:   [93.055 µs 94.781 µs 96.583 µs]

hex::decode_to_slice    time:   [31.949 µs 33.509 µs 35.285 µs]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

rustc_hex::from_hex     time:   [105.99 µs 108.05 µs 110.11 µs]
```

<div align="right">

#### License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>

</div>
