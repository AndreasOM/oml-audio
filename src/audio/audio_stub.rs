
use crate::FileLoader;
use crate::Music;
use crate::SoundBank;
//use crate::SoundPool;

use std::time::Instant;	// temporary, we get higher precision by calculating from the audio callbacks

#[derive(Debug)]
pub struct AudioStub {
	last_now:		Instant,
	capture_buffer: Vec< f32 >,
}

impl AudioStub {
	pub fn new() -> Self {
		Self {
	        last_now:		Instant::now(),
	        capture_buffer:	Vec::new(),
		}
	}

	pub fn update( &mut self ) -> f64 {
        let timestep = self.last_now.elapsed().as_secs_f64();
        self.last_now = Instant::now();

		timestep
	}

	pub fn load_music( &mut self, fileloader: &mut impl FileLoader, filename: &str ) -> bool {
//		self.music.load( fileloader, filename )
		true
	}

	pub fn play_music( &mut self ) {
//		self.music.play();
	}

	pub fn load_sound_bank( &mut self, fileloader: &mut impl FileLoader, filename: &str ) {
//		self.sound_bank.load( fileloader, filename )
	}

	pub fn play_sound( &mut self, name: &str ) {
//		self.sound_bank.play( name );
	}

	pub fn capture( &mut self, size: usize ) {
	}

	pub fn capture_buffer_slice( &self ) -> &[f32] {
		self.capture_buffer.as_slice()
	}

}
