
use crate::WavFile;

#[derive(Debug)]
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

	pub fn is_playing( &self ) -> bool {
		!self.done
	}

	pub fn play( &mut self ) {
		self.pos = 0;
		self.done = false;
	}

	pub fn stop( &mut self ) {
		self.done = true;
	}

	pub fn set_current_time( &mut self, time: f64 ) {
		self.pos = 0;	// :TODO:
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
