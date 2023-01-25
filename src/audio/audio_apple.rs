//use crate::SoundPool;
use std::time::Instant;

use crate::music::MusicApple;
use crate::AudioBackend;
use crate::FileLoader;
use crate::SoundBank; // temporary, we get higher precision by calculating from the audio callbacks

#[derive(Debug)]
pub struct AudioApple {
	music:          MusicApple,
	sound_bank:     SoundBank,
	last_now:       Instant,
	capture_buffer: Vec<f32>,
}

impl AudioApple {
	pub fn new() -> Self {
		Self {
			music:          MusicApple::new(),
			sound_bank:     SoundBank::new(),
			last_now:       Instant::now(),
			capture_buffer: Vec::new(),
		}
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

	pub fn capture(&mut self, _size: usize) {}

	pub fn capture_buffer_slice(&self) -> &[f32] {
		self.capture_buffer.as_slice()
	}
}

impl<F: crate::FileLoader> AudioBackend<F> for AudioApple {
	fn backend_type(&self) -> &'static str {
		"Apple"
	}
	fn start(&mut self) {}

	fn update(&mut self) -> f64 {
		let timestep = self.last_now.elapsed().as_secs_f64();
		self.last_now = Instant::now();

		self.music.update(timestep);
		self.sound_bank.update(timestep);
		timestep
	}
	fn load_sound_bank(&mut self, fileloader: &mut F, filename: &str) {
		self.sound_bank.load(fileloader, filename)
	}

	fn play_music(&mut self) {
		self.music.play();
	}
	fn pause_music(&mut self) {
		self.music.pause();
	}

	fn is_music_playing(&self) -> bool {
		self.music.is_playing()
	}

	fn play_sound(&mut self, name: &str) {
		self.sound_bank.play(name);
	}
	fn is_any_sound_playing(&self) -> bool {
		self.sound_bank.is_any_sound_playing()
	}
	fn load_music_native(&mut self, fileloader: &mut F, filename: &str) -> bool {
		let filename = format!("{}.mp3", filename);
		self.music.load(fileloader, &filename)
	}
}
