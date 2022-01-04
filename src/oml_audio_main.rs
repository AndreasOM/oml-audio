
use oml_audio::Music;
use oml_audio::Sound;

use oml_audio::fileloader::FileLoaderDisk;

use std::time::Instant;

pub fn main() {
	println!("oml-audio !");


	let mut fileloader = FileLoaderDisk::new( "./data" );

	let mut music = Music::new();
	music.load( &mut fileloader, "test.mp3" );
	music.play();

	let mut sound_coin = Sound::new();
	sound_coin.load( &mut fileloader, "coin", 10 );
	sound_coin.play( "coin" );

	let done = false;
	let mut last_now = Instant::now();
	let mut coin_timer = 0.0;
	let COIN_REPEAT = 0.5;
	while !done {
		let timestep = last_now.elapsed().as_secs_f64();
//		dbg!(timestep);
		last_now = Instant::now();

		music.update( timestep );
		sound_coin.update( timestep );

		coin_timer += timestep;

//		dbg!(coin_timer);
		if coin_timer > COIN_REPEAT {
			coin_timer -= COIN_REPEAT;
			sound_coin.play( "coin" );
		}

		std::thread::sleep( std::time::Duration::from_millis( 1000/60 ) );
	}
}