
use crate::FileLoader;


use std::collections::{
	HashMap,
	VecDeque,
};

use objc::*;
use objc::runtime::*;

#[derive(Debug)]
enum DropMode {
	Newest,
	Oldest,
	OlderThan,
}

#[derive(Debug)]
struct SoundPool {
	players:	VecDeque< *mut Object >,
	drop_mode:	DropMode,
}

impl SoundPool {
	pub fn new() -> Self {
		Self {
			players: 	VecDeque::new(),
			drop_mode:	DropMode::Oldest,
		}
	}

	fn load_data( fileloader: &mut impl FileLoader, filename: &str ) -> *mut Object {

		unsafe {
			let cls_nsdata = class!(NSData);

			let mut f = fileloader.open( &filename );
			let data: *mut Object = if f.is_valid() {
				println!("Loading Data from {}.", &filename);
				let mut buf = Vec::new();
				while !f.eof() {
					let c = f.read_u8();
					buf.push( c );
				}
				let slice = buf.as_slice();

				msg_send![ cls_nsdata, dataWithBytes:slice.as_ptr() length:slice.len() ]
			} else {
				msg_send![ cls_nsdata, data ]
			};

			let data_len: u64 = msg_send![ data, length ];
			dbg!(&data_len);

			data
		}
	}

	fn load_from_data(&mut self, data: *const Object ) -> bool {

		unsafe {
			let cls_nserror = class!(NSError);
			let error: *mut Object = msg_send![ cls_nserror, alloc ];

			let cls_avaudioplayer = class!(AVAudioPlayer);
			let player: *mut Object = msg_send![ cls_avaudioplayer, alloc ];
			let player: *mut Object = msg_send![ player, initWithData: data error: &error ];

			let prep_result: bool = msg_send![ player, prepareToPlay ];
			if prep_result {
				self.players.push_back( player );
				let _: () = msg_send![ player, setNumberOfLoops: 0 ];
				true
			} else {
				false
			}
//			let _: () = msg_send![ player, setVolume: 0.2 fadeDuration: 10.0 ];
		}

	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, name: &str, number: u16 ) -> bool {

		let extensions = [ ".caf", ".wav" ];

		let filename_maybe = extensions.iter().find_map(
			|e|{
				let filename = format!("{}{}", name, e);
				println!("Checking if {} exists:", filename );
				if fileloader.exists( &filename ) {
					Some( filename.to_owned() )
				} else {
					None
				}
			}
		);

		if let Some( filename ) = filename_maybe {
			let data = SoundPool::load_data( fileloader, &filename );


			for n in 0..number {
				if !self.load_from_data( data ) {
					println!("Couldn't read Sound {} from {}!", &name, &filename);
					return false
				}
			}
			dbg!( self.players.len() );
			true
		} else {
			println!("Sound {} not found", &name );
			false
		}
	}

	pub fn play( &mut self ) {
		if let Some( player ) = if let Some( &player ) = self.players.front( ) {
			unsafe {
				let playing: bool = msg_send![ player, isPlaying ];
				if !playing {
					println!("Using new player!");
					self.players.pop_front()
				} else {
					match self.drop_mode {
						DropMode::Newest => None,
						DropMode::Oldest => {
							println!("Reusing old player!");
							let _: () = msg_send![ player, stop ];
//							let _: () = msg_send![ player, setCurrentTime: 0.0 ];
							self.players.pop_front()
						},
						DropMode::OlderThan => {
							let current_time: f64 = msg_send![ player, currentTime ];
							if current_time > 0.5 {
								let _: () = msg_send![ player, stop ];
//								let _: () = msg_send![ player, setCurrentTime: 0.0 ];
								self.players.pop_front()
							} else {
								None
							}
						},
					}
				}
			}
		} else {
			println!("No player found!");
			None
		} {
			println!("Playing!");
			unsafe {
				let _: () = msg_send![ player, setCurrentTime: 0.0 ];
				let _: () = msg_send![ player, play ];
			}
			self.players.push_back( player );
		}
	}

	pub fn update( &mut self, _time_step: f64 ) {
	}
}

#[derive(Debug)]
pub struct SoundApple {
	sound_pools: HashMap< String, SoundPool>,
}

impl SoundApple {

	pub fn new() -> Self {
		Self {
			sound_pools: HashMap::new(),
		}
	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, name: &str, number: u16 ) -> bool {
		let mut sound_pool = SoundPool::new();
		if sound_pool.load( fileloader, name, number ) {
			self.sound_pools.insert( name.to_string(), sound_pool );
			true
		} else {
			false
		}
	}

	pub fn play( &mut self, name: &str ) {
		if let Some( sound_pool ) = self.sound_pools.get_mut( name ) {
			sound_pool.play();
		}
	}

	pub fn update( &mut self, time_step: f64 ) {
		for sound_pool in self.sound_pools.values_mut() {
			sound_pool.update( time_step );
		}
	}
}

