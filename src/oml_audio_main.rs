use oml_audio::fileloader::FileLoaderDisk;
use oml_audio::Audio;
use oml_audio::Music;
use oml_audio::SoundBank;

pub fn main() {
	println!("oml-audio !");

	let mut fileloader = FileLoaderDisk::new("./data");
	fileloader.enable_debug();

	let mut audio = Audio::new();
	//	audio.start();

	//	audio.load_music( &mut fileloader, "test.mp3" );
	audio.load_music_native(&mut fileloader, "test");

	audio.start();
	audio.play_music();

	audio.load_sound_bank(&mut fileloader, "test.omsb");

	//	sound_bank.play( "DEATH" );

	//	audio.play_sound( "MUSIC" );

	let done = false;
	let mut coin_timer = 0.0;
	let COIN_REPEAT = 0.5;

	let mut powerup_timer = 0.0;
	let POWERUP_REPEAT = 3.1;
	while !done {
		let timestep = audio.update();
		//		music.update( timestep );
		//		sound_bank.update( timestep );

		coin_timer += timestep;

		while coin_timer > COIN_REPEAT {
			coin_timer -= COIN_REPEAT;
			//			sound_bank.play( "PICKUP_COIN" );
			audio.play_sound("PICKUP_COIN");
		}

		powerup_timer += timestep;
		while powerup_timer > POWERUP_REPEAT {
			powerup_timer -= POWERUP_REPEAT;
			//			sound_bank.play( "POWERUP" );
			audio.play_sound("POWERUP");
		}

		std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
	}
}
