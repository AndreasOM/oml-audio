//use crate::SoundPool;
use std::time::Instant;

use crate::FileLoader;
use crate::Music;
use crate::SoundBank; // temporary, we get higher precision by calculating from the audio callbacks

#[derive(Debug)]
pub struct AudioApple {
	music:          Music,
	sound_bank:     SoundBank,
	last_now:       Instant,
	capture_buffer: Vec<f32>,
}

impl AudioApple {
	pub fn new() -> Self {
		Self {
			music:          Music::new(),
			sound_bank:     SoundBank::new(),
			last_now:       Instant::now(),
			capture_buffer: Vec::new(),
		}
	}

	pub fn start(&mut self) {}

	pub fn update(&mut self) -> f64 {
		let timestep = self.last_now.elapsed().as_secs_f64();
		self.last_now = Instant::now();

		self.music.update(timestep);
		self.sound_bank.update(timestep);
		timestep
	}

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

	pub fn load_music(&mut self, fileloader: &mut impl FileLoader, filename: &str) -> bool {
		self.music.load(fileloader, filename)
	}

	pub fn play_music(&mut self) {
		self.music.play();
	}

	pub fn pause_music(&mut self) {
		self.music.pause();
	}

	pub fn load_sound_bank(&mut self, fileloader: &mut impl FileLoader, filename: &str) {
		self.sound_bank.load(fileloader, filename)
	}

	pub fn play_sound(&mut self, name: &str) {
		self.sound_bank.play(name);
	}

	pub fn is_any_sound_playing(&self) -> bool {
		self.sound_bank.is_any_sound_playing()
	}

	pub fn capture(&mut self, size: usize) {}

	pub fn capture_buffer_slice(&self) -> &[f32] {
		self.capture_buffer.as_slice()
	}
}
