<div align="center">

# array-bytes
### A Collection of Array/Bytes/Hex Utilities.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/array-bytes/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/docsrs/array-bytes)](https://docs.rs/array-bytes)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/array-bytes)](https://github.com/hack-ink/array-bytes)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/array-bytes?color=red&style=plastic)](https://github.com/hack-ink/array-bytes)

</div>

## Abilities
#### `TryFromHex` trait
- Convert `Hex` to `Num`
	- type `AsRef<str> -> isize`
	- type `AsRef<str> -> i8`
	- type `AsRef<str> -> i16`
	- type `AsRef<str> -> i32`
	- type `AsRef<str> -> i64`
	- type `AsRef<str> -> i128`
	- type `AsRef<str> -> usize`
	- type `AsRef<str> -> u8`
	- type `AsRef<str> -> u16`
	- type `AsRef<str> -> u32`
	- type `AsRef<str> -> u64`
	- type `AsRef<str> -> u128`

#### `bytes` prefixed functions
- Convert `Bytes` to `Hex`
  - type `AsRef<[u8]> -> String`

#### `hex` prefixed functions
- Convert `HexBytes` to `Hex`
  - type `&[u8] -> &str`
  - e.g. `b"0x..." -> "0x..."`
- Transform `Hex` from `Array`
  - type `&str -> [u8; N]`
- Convert `Hex` to `Bytes`
  - type  `AsRef<str> -> Vec<u8>`
- Convert `Hex` to `Slice`
  - type `AsRef<str> -> &[u8]`
- Transform `Hex` to `T`
  - type `AsRef<str> -> T`
  - e.g. `"0x..." -> [u8; 20] -> H160`

#### `slice` prefixed functions
- Build fixed length `Array` from `Slice`
  - type `&[T] -> [T; N]`
- Transform `Slice` to `G`
  - type `&[T] -> G`
  - e.g. `&[0_u8, ...] -> [u8; 20] -> H160`

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
<div align="right"><sub>Friday, November 25th, 2022</sub></div>

```rs
array_bytes::bytes2hex  time:   [38.078 µs 38.126 µs 38.177 µs]
                        change: [-0.5147% -0.2140% +0.1154%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  11 (11.00%) high severe

hex::encode             time:   [136.19 µs 136.65 µs 137.14 µs]
                        change: [-0.3002% -0.0359% +0.2459%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  1 (1.00%) high severe

rustc_hex::to_hex       time:   [79.155 µs 79.268 µs 79.398 µs]
                        change: [-2.9058% -1.8791% -0.9713%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

faster_hex::hex_string  time:   [18.483 µs 18.524 µs 18.565 µs]
                        change: [-0.6456% -0.3159% -0.0313%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  8 (8.00%) high mild
  2 (2.00%) high severe

faster_hex::hex_encode_fallback
                        time:   [18.497 µs 18.528 µs 18.561 µs]
                        change: [-11.257% -4.9349% -0.9424%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

array_bytes::hex2bytes  time:   [224.34 µs 224.59 µs 224.86 µs]
                        change: [-1.7703% -1.2368% -0.7744%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

array_bytes::hex2bytes_unchecked
                        time:   [222.78 µs 223.07 µs 223.39 µs]
                        change: [-0.5184% -0.1710% +0.1429%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

array_bytes::hex2slice  time:   [211.37 µs 211.49 µs 211.62 µs]
                        change: [-3.1739% -2.1127% -1.2688%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  4 (4.00%) high severe

array_bytes::hex2slice_unchecked
                        time:   [212.00 µs 212.34 µs 212.71 µs]
                        change: [-0.8427% -0.5482% -0.2810%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe

hex::decode             time:   [244.37 µs 244.78 µs 245.25 µs]
                        change: [-1.4130% -1.0496% -0.7133%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

hex::decode_to_slice    time:   [166.67 µs 166.90 µs 167.16 µs]
                        change: [+0.1484% +0.3293% +0.5160%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

rustc_hex::from_hex     time:   [176.56 µs 177.79 µs 179.13 µs]
                        change: [+1.4009% +2.5404% +3.5866%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

faster_hex::hex_decode  time:   [39.127 µs 39.342 µs 39.582 µs]
                        change: [-0.0442% +0.3160% +0.6546%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) low severe
  8 (8.00%) high mild
  7 (7.00%) high severe

faster_hex::hex_decode_unchecked
                        time:   [16.429 µs 16.479 µs 16.538 µs]
                        change: [+0.5738% +0.9176% +1.3113%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

faster_hex::hex_decode_fallback
                        time:   [16.422 µs 16.440 µs 16.460 µs]
                        change: [+0.3595% +0.6397% +0.9141%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```

<div align="right">

#### License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>

</div>
