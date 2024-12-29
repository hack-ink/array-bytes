//! The origin benchmark comes from [rust-hex](https://github.com/KokaKiwi/rust-hex/blob/main/benches/hex.rs).
//! Thanks for their previous works.

// crates.io
use array_bytes::{Dehexify, Hexify};
use criterion::Criterion;
use hex_crate as hex;
use rustc_hex::{FromHex, ToHex};

const DATA: &[u8] = include_bytes!("../LICENSE-GPL3");

fn bench_encode(c: &mut Criterion) {
	c.bench_function("array_bytes::Hexify::hexify", |b| b.iter(|| DATA.hexify()));

	c.bench_function("const_hex::encode", |b| b.iter(|| const_hex::encode(DATA)));

	c.bench_function("faster_hex::hex_string", |b| b.iter(|| faster_hex::hex_string(DATA)));

	c.bench_function("faster_hex::hex_encode_fallback", |b| {
		b.iter(|| {
			let mut dst = vec![0; DATA.len() * 2];

			faster_hex::hex_encode_fallback(DATA, &mut dst);

			dst
		})
	});

	c.bench_function("hex::encode", |b| b.iter(|| hex::encode(DATA)));

	c.bench_function("rustc_hex::to_hex", |b| b.iter(|| DATA.to_hex::<String>()));
}

fn bench_decode(c: &mut Criterion) {
	let hex = DATA.hexify();

	c.bench_function("array_bytes::Dehexify::dehexify", |b| {
		b.iter(|| <Vec<u8>>::dehexify(&hex).unwrap())
	});
	c.bench_function("array_bytes::dehexify_slice_mut", |b| {
		b.iter(|| {
			let mut v = vec![0; DATA.len()];

			array_bytes::dehexify_slice_mut(&hex, &mut v).unwrap();

			v
		})
	});

	c.bench_function("const_hex::decode", |b| b.iter(|| const_hex::decode(&hex).unwrap()));

	c.bench_function("faster_hex::hex_decode", |b| {
		let mut v = vec![0; DATA.len()];

		b.iter(|| faster_hex::hex_decode(hex.as_bytes(), &mut v).unwrap())
	});
	c.bench_function("faster_hex::hex_decode_unchecked", |b| {
		let mut dst = vec![0; DATA.len()];

		b.iter(|| faster_hex::hex_decode_unchecked(hex.as_bytes(), &mut dst))
	});
	c.bench_function("faster_hex::hex_decode_fallback", |b| {
		let mut dst = vec![0; DATA.len()];

		b.iter(|| faster_hex::hex_decode_fallback(hex.as_bytes(), &mut dst))
	});

	c.bench_function("hex::decode", |b| b.iter(|| hex::decode(&hex).unwrap()));
	c.bench_function("hex::decode_to_slice", |b| {
		b.iter(|| {
			let mut v = vec![0; DATA.len()];

			hex::decode_to_slice(&hex, &mut v).unwrap();

			v
		})
	});

	c.bench_function("rustc_hex::from_hex", |b| b.iter(|| hex.from_hex::<Vec<u8>>().unwrap()));
}

criterion::criterion_group!(benches, bench_encode, bench_decode);
criterion::criterion_main!(benches);
