
use oml_audio::Music;
use oml_audio::SoundBank;

use oml_audio::fileloader::FileLoaderDisk;

use std::time::Instant;

pub fn main() {
	println!("oml-audio !");


	let mut fileloader = FileLoaderDisk::new( "./data" );

	let mut music = Music::new();
	music.load( &mut fileloader, "test.mp3" );
	music.play();


	let mut sound_bank = SoundBank::new();
	sound_bank.load( &mut fileloader, "test.omsb" );

	sound_bank.play( "DEATH" );

	let done = false;
	let mut last_now = Instant::now();
	let mut coin_timer = 0.0;
	let COIN_REPEAT = 0.5;

	let mut powerup_timer = 0.0;
	let POWERUP_REPEAT = 3.1;
	while !done {
		let timestep = last_now.elapsed().as_secs_f64();
		last_now = Instant::now();

		music.update( timestep );
		sound_bank.update( timestep );

		coin_timer += timestep;

		while coin_timer > COIN_REPEAT {
			coin_timer -= COIN_REPEAT;
			sound_bank.play( "PICKUP_COIN" );
		}

		powerup_timer += timestep;
		while powerup_timer > POWERUP_REPEAT {
			powerup_timer -= POWERUP_REPEAT;
			sound_bank.play( "POWERUP" );
		}

		std::thread::sleep( std::time::Duration::from_millis( 1000/60 ) );
	}
}