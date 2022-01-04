
use oml_audio::Music;

use oml_audio::fileloader::FileLoaderDisk;

use std::time::Instant;

pub fn main() {
	println!("oml-audio !");

	let mut music = Music::new();

	let mut fileloader = FileLoaderDisk::new( "." );

	music.load( &mut fileloader, "data/test.mp3" );
	music.play();

	let done = false;
	let mut last_now = Instant::now();
	while !done {
		let timestep = last_now.elapsed().as_secs_f64();
		dbg!(timestep);
		last_now = Instant::now();
		music.update( timestep );
		std::thread::sleep( std::time::Duration::from_millis( 1000/60 ) );
	}
}