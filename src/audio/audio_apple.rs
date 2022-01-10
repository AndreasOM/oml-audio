
use crate::FileLoader;
use crate::Music;
use crate::SoundBank;
//use crate::SoundPool;

pub struct AudioApple {
	music:			Music,
	sound_bank:		SoundBank,
}

impl AudioApple {
	pub fn new() -> Self {
		Self {
			music:			Music::new(),
			sound_bank:		SoundBank::new(),
		}
	}

	pub fn update( &mut self, timestep: f64 ) {
		self.music.update( timestep );
		self.sound_bank.update( timestep );	
	}

	pub fn load_music( &mut self, fileloader: &mut impl FileLoader, filename: &str ) -> bool {
		self.music.load( fileloader, filename )
	}

	pub fn play_music( &mut self ) {
		self.music.play();
	}

	pub fn load_sound_bank( &mut self, fileloader: &mut impl FileLoader, filename: &str ) {
		self.sound_bank.load( fileloader, filename )
	}

	pub fn play_sound( &mut self, name: &str ) {
		self.sound_bank.play( name );
	}
}
