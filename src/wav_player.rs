
use crate::WavFile;

pub struct WavPlayer {
	pos: usize,
	done: bool,
}

impl WavPlayer {

	pub fn new() -> Self {
		Self {
			pos: 0,
			done: true,
		}
	}

	pub fn done( &self ) -> bool {
		self.done
	}

	pub fn play( &mut self ) {
		self.pos = 0;
		self.done = false;
	}

	pub fn next_sample( &mut self, wav_file: &WavFile ) -> f32 {
		let data = wav_file.data();
		if self.pos >= data.len() {
			self.done = true;
			0.0
		} else {
			let v = data[ self.pos ];
			self.pos += 1;
			v
		}
	}
}
