<div align="center">

<!-- Logo -->
<!-- ![array-bytes]() -->

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

#### `bytes` prefixed functions
- Convert `Bytes` to `Hex`
  - type `&[u8] -> String`

#### `hex` prefixed functions
- Convert `HexBytes` to `Hex`
  - type `&[u8] -> &str`
  - e.g. `b"0x..." -> "0x..."`
- Build fixed length `Array` from `Hex`
  - type `&str -> [u8; N]`
- Convert `Hex` to `Bytes`
  - type  `&str -> Vec<u8>`
- Transform `Hex` to `T`
  - type `&str -> T`
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
<div align="right"><sub>Friday, September 9, 2022</sub></div>

```rs
array_bytes::bytes2hex  time:   [37.577 µs 37.755 µs 37.914 µs]
                        change: [+0.1343% +0.9208% +1.7020%] (p = 0.03 < 0.05)
                        Change within noise threshold.

hex::encode             time:   [56.832 µs 57.056 µs 57.258 µs]
                        change: [-0.3311% +0.1929% +0.7251%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

rustc_hex::to_hex       time:   [33.522 µs 33.611 µs 33.690 µs]
                        change: [+0.0534% +0.4709% +0.9031%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) low mild

faster_hex::hex_string  time:   [7.7683 µs 7.8169 µs 7.8819 µs]
                        change: [+3.7564% +4.6695% +5.6205%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) low severe
  5 (5.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

faster_hex::hex_encode_fallback
                        time:   [7.7536 µs 7.7740 µs 7.7936 µs]
                        change: [+1.3656% +1.9964% +2.6314%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

array_bytes::hex2bytes  time:   [136.78 µs 137.66 µs 138.57 µs]
                        change: [+0.1075% +0.8175% +1.5436%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) low mild
  1 (1.00%) high severe

array_bytes::hex2bytes_unchecked
                        time:   [103.12 µs 103.49 µs 103.82 µs]
                        change: [-0.7568% -0.2119% +0.3349%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild

hex::decode             time:   [105.65 µs 106.15 µs 106.65 µs]
                        change: [-1.4824% -0.9722% -0.4135%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

rustc_hex::from_hex     time:   [66.218 µs 66.473 µs 66.750 µs]
                        change: [-0.0431% +0.6585% +1.4552%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

faster_hex::hex_decode  time:   [16.136 µs 16.211 µs 16.324 µs]
                        change: [-2.4694% -1.9531% -1.4089%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

faster_hex::hex_decode_unchecked
                        time:   [6.7977 µs 6.8215 µs 6.8475 µs]
                        change: [-1.4235% -0.5587% +0.2182%] (p = 0.20 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

faster_hex::hex_decode_fallback
                        time:   [6.7799 µs 6.8003 µs 6.8209 µs]
                        change: [-1.9470% -1.3817% -0.8203%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```

<div align="right">

#### License
<sup>Licensed under either of <a href="LICENSE-APACHE">Apache-2.0</a> or <a href="LICENSE-GPL3">GPL-3.0</a> at your option.</sup>

</div>
