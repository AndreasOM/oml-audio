
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
		let mut file = SoundBankFile::new();
		file.load( fileloader, filename );
//		dbg!( &file );
		for e in file.entries() {
//			dbg!( &e );

			self.load_sound( fileloader, &e.filename, &e.id, e.max_instances );
			// :TODO: loop & drop mode
		}
	}

	pub fn play( &mut self, name: &str ) {
		if let Some( sound ) = self.sound_pools.get_mut( name ) {
			sound.play();
		}
	}

	pub fn is_any_sound_playing( &self ) -> bool {
		for sound in self.sound_pools.values() {
			if sound.is_any_sound_playing() {
				return true
			}
		}

		false
	}

	pub fn update( &mut self, timestep: f64 ) {
		for sound in self.sound_pools.values_mut() {
			sound.update( timestep );
		}
	}

	pub fn fill_slice( &mut self, slice: &mut [f32] ) {
		for p in self.sound_pools.values_mut() {
			p.fill_slice( slice );
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



#[derive(Debug,Clone)]
pub enum DropMode {
	Drop,
	Oldest,
}

#[derive(Debug,Clone)]
pub struct Entry {
	id: String,
	id_crc: u32,
	filename: String,
	max_instances: u16,
	drop_mode: DropMode,
	do_loop: bool,
}


#[derive(Debug)]
pub struct SoundBankFile {
	entries: Vec<Entry>,
}

impl SoundBankFile {
	pub fn new() -> Self {
		Self {
			entries: vec![],
		}
	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, filename: &str ) -> bool {

		let mut f = fileloader.open( &filename );
		if ! f.is_valid() {
			println!("Couldn't open file: {}", filename);
			return false;
		}

		let s0 = f.read_u16();
		if 0x4f53 != s0 {
			println!("Wrong signature {:#02x}", s0 );
			return false;
		}

		let v0 = f.read_u16();
		if 0x0001 != v0 {
			println!("Wrong version {:#02x}", v0 );
			return false;
		}

		for b in &[ 0x4f, 0x4d, 0x53, 0x4e, 0x44, 0x42, 0x4e ] {
			let b1 = f.read_u8();
			if *b != b1 {
				println!("Wrong signature {:#02x} != {:#02x}", b, b1 );
				return false;				
			}
		}

		let c = f.read_u8();
		if c != b'K' /*&& c != b'Z'*/ { // :TODO:
			println!("Unsupported compression mode {}", c );
			return false;
		}

		for b in &[ 0x03, 0x00, 0x00, 0x00 ] {
			let b1 = f.read_u8();
			if *b != b1 {
				println!("Wrong signature {:#02x} != {:#02x}", b, b1 );
				return false;				
			}
		}

		let ne = f.read_u16();
//		println!("{} entries", ne);

		for e in 0..ne {
//			println!("Loading entry {} of {}", e, ne );
			let id_crc = f.read_u32();
//			println!("CRC {:#02x}", id_crc );

			let mut filename_bytes = vec![0;32];
			let mut filename_len = 0;
			for n in 0..32 {
				let b = f.read_u8();
				filename_bytes[ n ] = b;
				if b != 0 {
					filename_len = n+1;
				}
			}

			filename_bytes.resize( filename_len, 0 );

			let filename = String::from_utf8_lossy( &filename_bytes );

//			dbg!( &filename );

			let mut name_bytes = vec![0;32];
			let mut name_len = 0;
			for n in 0..32 {
				let b = f.read_u8();
				name_bytes[ n ] = b;
				if b != 0 {
					name_len = n+1;
				}
			}

			name_bytes.resize( name_len, 0 );

			let name = String::from_utf8_lossy( &name_bytes );

//			dbg!( &name );

			let max_instances = f.read_u16();
			let drop_mode = f.read_u16();
			let flags = f.read_u32();


			let entry = Entry {
				id: name.to_string(),
				id_crc,
				filename: filename.to_string(),
				max_instances,
				drop_mode: DropMode::Oldest,	// :TODO:
				do_loop: false,					// :TODO:
			};

			self.entries.push( entry );
		}

		true
	}

	pub fn entries( self ) -> Vec<Entry> {
		self.entries
	}

}

