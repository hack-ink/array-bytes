#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
	let _ = array_bytes::bytes2hex("", data);
	let _ = array_bytes::hex_bytes2hex_str(data);
	let _ = array_bytes::hex2bytes(&String::from_utf8_lossy(data));
});
