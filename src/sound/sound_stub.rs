use crate::FileLoader;

#[derive(Debug)]
pub struct SoundPoolStub {
	debug: bool,
}

impl SoundPoolStub {
	pub fn new() -> Self {
		Self { debug: false }
	}

	pub fn load(&mut self, fileloader: &mut impl FileLoader, name: &str, number: u16) -> bool {
		true
	}

	pub fn play(&mut self) {}

	pub fn is_any_sound_playing(&self) -> bool {
		false
	}

	pub fn update(&mut self, _time_step: f64) {}

	pub fn fill_slice(&mut self, _slice: &mut [f32]) {}

	pub fn enable_debug(&mut self) {
		self.debug = true;
	}
	pub fn disable_debug(&mut self) {
		self.debug = false;
	}
}
