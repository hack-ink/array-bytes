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
  - type `AsRef<[u8]> -> [u8; N]`, `N = { [1, 64], 128, 256, 512 }`
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
  - type `[u8; N] -> String`, `N = { [1, 64], 128, 256, 512 }`
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
- `#[serde(deserialize_with = "array_bytes::de_hex2num")]`
  - type `S -> Num`
  - e.g. `"0xA" -> 10_u32`
- `#[serde(deserialize_with = "array_bytes::de_hex2bytes")]`
  - type `S -> Vec<u8>`
  - e.g. `"0x00" -> vec![0_u8]`

## Benchmark results
<div align="right"><sub>Tuesday, January 9th, 2024</sub></div>

```rs
array_bytes::bytes2hex  time:   [30.487 µs 30.513 µs 30.543 µs]
                        change: [-12.758% -7.1673% -2.3095%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

const_hex::encode       time:   [2.1197 µs 2.2245 µs 2.3208 µs]
                        change: [+25.796% +31.010% +36.449%] (p = 0.00 < 0.05)
                        Performance has regressed.

faster_hex::hex_string  time:   [13.666 µs 13.711 µs 13.754 µs]
                        change: [-0.3508% +0.0892% +0.5043%] (p = 0.68 > 0.05)
                        No change in performance detected.

faster_hex::hex_encode_fallback
                        time:   [13.476 µs 13.519 µs 13.564 µs]
                        change: [-0.1799% +0.2323% +0.6560%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

hex::encode             time:   [133.99 µs 135.65 µs 137.13 µs]
                        change: [-1.6763% +0.2181% +2.1203%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  13 (13.00%) low severe
  2 (2.00%) low mild

rustc_hex::to_hex       time:   [118.83 µs 124.46 µs 129.51 µs]
                        change: [-3.5525% +2.8439% +10.307%] (p = 0.42 > 0.05)
                        No change in performance detected.

array_bytes::hex2bytes  time:   [46.892 µs 47.510 µs 48.195 µs]
                        change: [-8.2282% -6.5411% -4.6367%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

array_bytes::hex2bytes_unchecked
                        time:   [73.450 µs 73.842 µs 74.251 µs]
                        change: [+0.5740% +1.3693% +2.1806%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  11 (11.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

array_bytes::hex2slice  time:   [57.825 µs 57.915 µs 58.007 µs]
                        change: [-0.9051% -0.6249% -0.3523%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

array_bytes::hex2slice_unchecked
                        time:   [73.574 µs 73.917 µs 74.281 µs]
                        change: [-4.9137% -4.1840% -3.4519%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe

const_hex::decode       time:   [15.849 µs 15.887 µs 15.924 µs]
                        change: [-3.1788% -2.1280% -1.2019%] (p = 0.00 < 0.05)
                        Performance has improved.

faster_hex::hex_decode  time:   [31.735 µs 31.764 µs 31.800 µs]
                        change: [-0.7403% -0.5216% -0.2674%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

faster_hex::hex_decode_unchecked
                        time:   [13.059 µs 13.098 µs 13.145 µs]
                        change: [-0.7125% -0.3968% -0.1100%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

faster_hex::hex_decode_fallback
                        time:   [13.074 µs 13.090 µs 13.108 µs]
                        change: [-1.9404% -1.5652% -1.1912%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

hex::decode             time:   [131.19 µs 132.57 µs 134.26 µs]
                        change: [+1.9126% +3.2801% +4.8702%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  15 (15.00%) high severe

hex::decode_to_slice    time:   [57.577 µs 58.018 µs 58.447 µs]
                        change: [+0.0185% +1.1253% +2.2104%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

rustc_hex::from_hex     time:   [130.19 µs 130.86 µs 131.48 µs]
                        change: [-1.8542% -1.4374% -0.9862%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

<div align="right">

#### License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>

</div>
