
use crate::FileLoader;


#[derive(Debug)]
pub struct SoundPoolMiniaudio {
	debug: bool,
}

impl SoundPoolMiniaudio {

	pub fn new() -> Self {
		Self {
			debug: false,
		}
	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, name: &str, number: u16 ) -> bool {
		true
	}

	pub fn play( &mut self ) {
	}

	pub fn update( &mut self, _time_step: f64 ) {
	}

	pub fn enable_debug( &mut self ) {
		self.debug = true;
	}
	pub fn disable_debug( &mut self ) {
		self.debug = false;
	}

}
