
use crate::FileLoader;

use crate::sound::DropMode;

use crate::{
	WavFile,
	WavPlayer,
};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct SoundPoolMiniaudio {
	debug:			bool,
	wav_file:		WavFile,
	players:		VecDeque< WavPlayer >,
	drop_mode:		DropMode,
}

impl SoundPoolMiniaudio {

	pub fn new() -> Self {
		Self {
			debug:			false,
			wav_file:		WavFile::new(),
			players:		VecDeque::new(),
			drop_mode:		DropMode::Oldest,
		}
	}

	pub fn load( &mut self, fileloader: &mut impl FileLoader, name: &str, number: u16 ) -> bool {
		let filename = format!("{}{}", name, ".wav");
		self.wav_file.load( fileloader, &filename );

		for n in 0..number {
			let player = WavPlayer::new();
			self.players.push_back( player );
		}

		true
	}

	pub fn play( &mut self ) {
		if let Some( mut player ) = if let Some( player ) = self.players.front( ) {
			let playing: bool = player.is_playing();
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
						/*
						let current_time: f64 = msg_send![ player, currentTime ];
						if current_time > 0.5 {
//								let _: () = msg_send![ player, stop ];
//								let _: () = msg_send![ player, setCurrentTime: 0.0 ];
							self.players.pop_front()
						} else {
							None
						}
						*/
						None
					},
				}
			}
		} else {
			if self.debug { println!("No player found!"); }
			None
		} {
			if self.debug { println!("Playing!"); }
			player.stop();
			player.set_current_time( 0.0 );
			player.play();
			self.players.push_back( player );
		}
	}

	pub fn update( &mut self, _time_step: f64 ) {
	}

	pub fn next_sample( &mut self ) -> f32 {
		let mut v = 0.0;
		for p in self.players.iter_mut() {
			if p.is_playing() {
				v += p.next_sample( &self.wav_file );
			}
		}
		v
	}

	pub fn enable_debug( &mut self ) {
		self.debug = true;
	}
	pub fn disable_debug( &mut self ) {
		self.debug = false;
	}

}

