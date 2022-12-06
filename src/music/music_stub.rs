use crate::FileLoader;

#[derive(Debug)]
pub struct MusicStub {}

impl MusicStub {
	pub fn new() -> Self {
		Self {}
	}
	pub fn load(&mut self, fileloader: &mut impl FileLoader, filename: &str) -> bool {
		true
	}

	pub fn play(&mut self) {}

	pub fn pause(&mut self) {}

	pub fn stop(&mut self) {}

	pub fn update(&mut self, _time_step: f64) {}
}
