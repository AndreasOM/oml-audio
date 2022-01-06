
use std::collections::HashMap;

use crate::FileLoader;
use crate::SoundPool;

#[derive(Debug)]
pub struct SoundBank {
	sound_pools: HashMap< String, SoundPool >,
	debug:			bool,
}

impl SoundBank {
	pub fn new() -> Self {
		Self {
			sound_pools: HashMap::new(),
			debug: false,
		}
	}

	pub fn load_sound( &mut self, fileloader: &mut impl FileLoader, filename: &str, name: &str, number: u16 ) {
		let mut sound_pool = SoundPool::new();
		if self.debug {
			sound_pool.enable_debug();
		} else {
			sound_pool.disable_debug();			
		}
		sound_pool.load( fileloader, filename, number );
		if let Some( old_sound ) = self.sound_pools.insert( name.to_string(), sound_pool ) {
			// :TODO: cleanup old sound_pool, maybe
		}
	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, filename: &str ) {
		// :HACK:
		self.load_sound( fileloader, "coin", "COIN", 10 );
		self.load_sound( fileloader, "powerup", "POWERUP", 2 );
		self.load_sound( fileloader, "death", "DEATH", 1 );
	}

	pub fn play( &mut self, name: &str ) {
		if let Some( sound ) = self.sound_pools.get_mut( name ) {
			sound.play();
		}
	}

	pub fn update( &mut self, timestep: f64 ) {
		for sound in self.sound_pools.values_mut() {
			sound.update( timestep );
		}
	}

	pub fn enable_debug( &mut self ) {
		self.debug = true;
		for sp in self.sound_pools.values_mut() {
			sp.enable_debug();
		}
	}
	pub fn disable_debug( &mut self ) {
		self.debug = false;
		for sp in self.sound_pools.values_mut() {
			sp.disable_debug();
		}
	}

}

