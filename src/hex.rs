mod hexify;
pub use hexify::*;

mod dehexify;
pub use dehexify::*;

// self
#[cfg(test)] use crate::prelude::*;

#[test]
fn random_input_should_work() {
	let data = [
		include_bytes!("../Cargo.lock").as_slice(),
		include_bytes!("../LICENSE-APACHE2"),
		include_bytes!("../LICENSE-GPL3"),
	]
	.concat();

	[8, 16, 32, 64, 128, 256, 512, 1024].into_iter().for_each(|chunks_size| {
		let mut data_pieces = Vec::new();

		data.chunks(chunks_size).enumerate().for_each(|(i, chunk)| {
			let data_piece = match i % 4 {
				0 => chunk.hexify(),
				1 => chunk.hexify_upper(),
				2 => chunk.hexify_prefixed(),
				3 => chunk.hexify_prefixed_upper(),
				_ => unreachable!(),
			};

			data_pieces.push(data_piece);
		});

		let data_pieces = data_pieces
			.into_iter()
			.map(|piece| match strip_0x(piece.as_bytes()).len() {
				8 => <[u8; 4]>::dehexify(&piece).unwrap().to_vec(),
				32 => <[u8; 16]>::dehexify(&piece).unwrap().to_vec(),
				64 => <[u8; 32]>::dehexify(&piece).unwrap().to_vec(),
				128 => <[u8; 64]>::dehexify(&piece).unwrap().to_vec(),
				256 => <[u8; 128]>::dehexify(&piece).unwrap().to_vec(),
				512 => <[u8; 256]>::dehexify(&piece).unwrap().to_vec(),
				1024 => <[u8; 512]>::dehexify(&piece).unwrap().to_vec(),
				2048 => <[u8; 1024]>::dehexify(&piece).unwrap().to_vec(),
				_ => <Vec<u8>>::dehexify(&piece).unwrap(),
			})
			.collect::<Vec<_>>();

		assert_eq!(data_pieces.concat(), data);
	});
}
