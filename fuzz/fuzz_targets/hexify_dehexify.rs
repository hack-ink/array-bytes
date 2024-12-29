#![no_main]

// self
use array_bytes::{Dehexify, Hexify};

macro_rules! fuzz_dehexify {
	($data:ident,$($t:ident,)+) => {
		$({
			let _ = $t::dehexify($data);
		})+
	};
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
	fuzz_dehexify! {
		data,
		usize,
		u8,
		u16,
		u32,
		u64,
		u128,
	}

	let _ = data.hexify();
	let _ = <Vec<u8>>::dehexify(data);

	{
		let mut v = vec![0; data.len() / 2];
		let _ = array_bytes::dehexify_slice_mut(data, &mut v);
	}
});
