//use crate::SoundPool;
use std::time::Instant;

use crate::music::MusicStub;
use crate::AudioBackend;
use crate::FileLoader;
use crate::SoundBank; // temporary, we get higher precision by calculating from the audio callbacks

#[derive(Debug)]
pub struct AudioStub {
	last_now:       Instant,
	capture_buffer: Vec<f32>,
	sound_bank:     SoundBank,
}

impl AudioStub {
	pub fn new() -> Self {
		eprintln!("Warning: Using STUB for all oml-audio!\nSelect a backend via a feature.");
		Self {
			last_now:       Instant::now(),
			capture_buffer: Vec::new(),
			sound_bank:     SoundBank::new(),
		}
	}

	pub fn start(&mut self) {}

	pub fn get_sound_bank_mut(&mut self) -> &mut SoundBank {
		&mut self.sound_bank
	}

	pub fn fill_buffer(
		_sound_bank: &mut SoundBank,
		_producer: &mut ringbuf::Producer<f32>,
	) -> usize {
		0
	}

	pub fn drain_buffer(_consumer: &mut ringbuf::Consumer<f32>) -> usize {
		0
	}

	pub fn load_music(&mut self, _fileloader: &mut impl FileLoader, _filename: &str) -> bool {
		//		self.music.load( fileloader, filename )
		true
	}

	pub fn capture(&mut self, size: usize) {}

	pub fn capture_buffer_slice(&self) -> &[f32] {
		self.capture_buffer.as_slice()
	}
}

impl<F: crate::FileLoader> AudioBackend<F> for AudioStub {
	fn start(&mut self) {}

	fn update(&mut self) -> f64 {
		let timestep = self.last_now.elapsed().as_secs_f64();
		self.last_now = Instant::now();

		timestep
	}
	fn load_sound_bank(&mut self, _fileloader: &mut F, _filename: &str) {}

	fn play_music(&mut self) {}
	fn pause_music(&mut self) {}
	fn is_music_playing(&self) -> bool { false }

	fn play_sound(&mut self, _name: &str) {}
	fn is_any_sound_playing(&self) -> bool {
		false
	}
	fn load_music_native(&mut self, _fileloader: &mut F, _filename: &str) -> bool {
		true
	}
}
