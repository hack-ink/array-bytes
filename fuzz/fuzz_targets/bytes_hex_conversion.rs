#![no_main]

macro_rules! fuzz_try_from_hex {
	($data:ident,$($t:ident,)+) => {
		$({
			// array-bytes
			use array_bytes::TryFromHex;

			let _ = $t::try_from_hex($data);
		})+
	};
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
	fuzz_try_from_hex! {
		data,
		isize,
		i8,
		i16,
		i32,
		i64,
		i128,
		usize,
		u8,
		u16,
		u32,
		u64,
		u128,
	}

	let _ = array_bytes::bytes2hex("", data);
	let _ = array_bytes::hex_bytes2hex_str(data);
	let _ = array_bytes::hex2bytes(data);

	{
		let mut v = vec![0; data.len() / 2];
		let _ = array_bytes::hex2slice(data, &mut v);
	}
});
