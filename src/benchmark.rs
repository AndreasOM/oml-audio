#![feature(test)]

extern crate test;


pub fn main() {
	println!("oml-audio BENCHMARK -> run via cargo bench ...");
}

#[cfg(test)]
mod tests {
	use oml_audio::Audio;
	use oml_audio::fileloader::FileLoaderDisk;

	use ringbuf::RingBuffer;

	use super::*;
	use test::Bencher;

    #[bench]
    fn sound_fill_buffer( b: &mut Bencher ) {
		let mut fileloader = FileLoaderDisk::new( "./data" );
		let mut audio = Audio::new();
		audio.load_sound_bank( &mut fileloader, "test.omsb" );
//		audio.play_sound( "PICKUP_COIN" );

		let mut rb = RingBuffer::new( 4*4096 );
		let ( mut producer, mut consumer ) = rb.split();

    	b.iter( || {
			audio.play_sound( "PICKUP_COIN" );
			while audio.is_any_sound_playing() {
				Audio::fill_buffer( audio.get_sound_bank_mut(), &mut producer );
				Audio::drain_buffer( &mut consumer );
//				let timestep = audio.update();
			}
		} )
	}
}
