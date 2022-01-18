
use crate::FileLoader;

use crate::sound::DropMode;

use std::collections::{
	HashMap,
	VecDeque,
};

use objc::*;
use objc::runtime::*;

#[derive(Debug)]
pub struct SoundPoolApple {
	players:	VecDeque< *mut Object >,
	drop_mode:	DropMode,
	debug:		bool,
}

impl SoundPoolApple {
	pub fn new() -> Self {
		Self {
			players: 	VecDeque::new(),
			drop_mode:	DropMode::Oldest,
			debug:		false,
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

			/*
			let data_len: u64 = msg_send![ data, length ];
			dbg!(&data_len);
			*/

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
			let data = SoundPoolApple::load_data( fileloader, &filename );


			for n in 0..number {
				if !self.load_from_data( data ) {
					println!("Couldn't read Sound {} from {}!", &name, &filename);
					return false
				}
			}
			if self.debug { dbg!( self.players.len() ); }
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
				if self.debug { dbg!(playing); }
				if !playing {
					if self.debug { println!("Using new player!"); }
					self.players.pop_front()
				} else {
					if self.debug { println!("No new player found!"); }
					match self.drop_mode {
						DropMode::Newest => None,
						DropMode::Oldest => {
							if self.debug { println!("Reusing old player!"); }
//							let _: () = msg_send![ player, stop ];
//							let _: () = msg_send![ player, setCurrentTime: 0.0 ];
							self.players.pop_front()
						},
						DropMode::OlderThan => {
							let current_time: f64 = msg_send![ player, currentTime ];
							if current_time > 0.5 {
//								let _: () = msg_send![ player, stop ];
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
			if self.debug { println!("No player found!"); }
			None
		} {
			if self.debug { println!("Playing!"); }
			unsafe {
				let _: () = msg_send![ player, stop ];
				let _: () = msg_send![ player, setCurrentTime: 0.0 ];
				let _: () = msg_send![ player, play ];
			}
			self.players.push_back( player );
		}
	}

	pub fn is_any_sound_playing( &self ) -> bool {
		for player in self.players.iter() {
			unsafe {
				let playing: bool = msg_send![ *player, isPlaying ];
				if playing {
					return true;
				}
			}			
		}

		false
	}

	pub fn update( &mut self, _time_step: f64 ) {
	}

	pub fn fill_slice( &mut self, _slice: &mut [f32] ) {
	}

	pub fn enable_debug( &mut self ) {
		self.debug = true;
	}
	pub fn disable_debug( &mut self ) {
		self.debug = false;
	}
}
