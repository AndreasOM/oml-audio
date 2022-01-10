
use oml_audio::Audio;
use oml_audio::Music;
use oml_audio::SoundBank;

use oml_audio::fileloader::FileLoaderDisk;

use std::time::Instant;

pub fn main() {
	println!("oml-audio !");


	let mut fileloader = FileLoaderDisk::new( "./data" );
	fileloader.enable_debug();

	let mut audio = Audio::new();

//	let mut music = Music::new();
//	music.load( &mut fileloader, "test.mp3" );
//	music.play();

	audio.load_music( &mut fileloader, "test.mp3" );
	audio.play_music();


	audio.load_sound_bank( &mut fileloader, "test.omsb" );

//	let mut sound_bank = SoundBank::new();
//	sound_bank.load( &mut fileloader, "test.omsb" );

//	sound_bank.play( "DEATH" );

	let done = false;
	let mut last_now = Instant::now();
	let mut coin_timer = 0.0;
	let COIN_REPEAT = 0.5;

	let mut powerup_timer = 0.0;
	let POWERUP_REPEAT = 3.1;
	while !done {
		let timestep = last_now.elapsed().as_secs_f64();
		last_now = Instant::now();

		audio.update( timestep );
//		music.update( timestep );
//		sound_bank.update( timestep );

		coin_timer += timestep;

		while coin_timer > COIN_REPEAT {
			coin_timer -= COIN_REPEAT;
//			sound_bank.play( "PICKUP_COIN" );
			audio.play_sound( "PICKUP_COIN" );
		}

		powerup_timer += timestep;
		while powerup_timer > POWERUP_REPEAT {
			powerup_timer -= POWERUP_REPEAT;
//			sound_bank.play( "POWERUP" );
			audio.play_sound( "POWERUP" );
		}

		std::thread::sleep( std::time::Duration::from_millis( 1000/60 ) );
	}
}