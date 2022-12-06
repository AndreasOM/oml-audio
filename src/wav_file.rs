use crate::FileLoader;

pub struct WavFile {
	data: Vec<f32>,
}

impl std::fmt::Debug for WavFile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WavFile")
			.field("data length", &self.data.len())
			.finish()
	}
}

impl WavFile {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}

	pub fn data(&self) -> &Vec<f32> {
		&self.data
	}

	pub fn load(&mut self, fileloader: &mut dyn FileLoader, filename: &str) -> bool {
		let mut f = fileloader.open(&filename);
		if !f.is_valid() {
			println!("Couldn't open file: {}", filename);
			return false;
		}
		// RIFF
		for b in &[b'R', b'I', b'F', b'F'] {
			let b1 = f.read_u8();
			if *b != b1 {
				println!("Wrong RIFF signature {:#02x} != {:#02x}", b, b1);
				return false;
			}
		}

		let size = f.read_u32();
		dbg!(size);
		/*
		if size + 8 != f.size() {
			println!("Wrong size {} != {}", size + 8, f.size() );
			return false;
		}
		*/

		// WAVE
		for b in &[b'W', b'A', b'V', b'E'] {
			let b1 = f.read_u8();
			if *b != b1 {
				println!("Wrong WAVE signature {:#02x} != {:#02x}", b, b1);
				return false;
			}
		}

		// fmt%20
		for b in &[b'f', b'm', b't', b' '] {
			let b1 = f.read_u8();
			if *b != b1 {
				println!("Wrong fmt signature {:#02x} != {:#02x}", b, b1);
				return false;
			}
		}

		let fmt_len = f.read_u32();
		dbg!(fmt_len);
		if fmt_len != 16 {
			println!("Unexpected fmt length {}", fmt_len);
			return false;
		}

		let fmt_type = f.read_u16();
		dbg!(fmt_type);
		if fmt_len != 16 {
			println!("Unexpected fmt type {} (not 1/PCM", fmt_type);
			return false;
		}

		let channels = f.read_u16();
		dbg!(channels);

		let sample_rate = f.read_u32();
		let byte_rate = f.read_u32();
		let block_size = f.read_u16();
		let bits_per_sample = f.read_u16();

		dbg!(sample_rate);
		dbg!(byte_rate);
		dbg!(block_size);
		dbg!(bits_per_sample);

		while !f.eof() {
			let mut chunk_type = [0u8; 4];

			for b in &mut chunk_type {
				*b = f.read_u8();
			}

			let chunk_size = f.read_u32();
			let chunk_type = std::str::from_utf8(&chunk_type).unwrap_or("");
			dbg!(chunk_type);

			match chunk_type {
				"data" => {
					println!("data chunk");
					//					let mut data = Vec::new();
					let blocks = chunk_size / block_size as u32;
					match bits_per_sample {
						16 => {
							for _ in 0..blocks {
								let w = f.read_u16();
								let w = unsafe { std::mem::transmute::<u16, i16>(w) };

								// :TODO:
								let f = 2.0 * (w as f32) / (0xffff as f32) - 0.0;
								self.data.push(f);
								if channels == 1 {
									// :HACK: just duplicate to enforce stereo
									self.data.push(f);
								}
							}
							println!("");
						},
						8 => {
							for _ in 0..blocks {
								let w = f.read_u8();
								// :TODO:
								let f = 2.0 * (w as f32) / (0xff as f32) - 1.0;
								self.data.push(f);
								if channels == 1 {
									// :HACK: just duplicate to enforce stereo
									self.data.push(f);
								}
							}
							println!("");
						},
						bps => {
							println!("Unhandled bits per sample {}", bps);
							return false;
						},
					}
					//					return false;
				},
				"" => {
					println!("None chunk ''");
					break;
				},
				_ => {
					println!(
						"Unhandled chunk type: {:?} size: skipping {}",
						chunk_type, chunk_size
					);
					for _ in 0..chunk_size {
						f.read_u8();
					}
				},
			};
		}
		false
	}
}
