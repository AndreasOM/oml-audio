
use crate::FileLoader;
use crate::fileloader::FileLoaderFile;

use lewton::inside_ogg::OggStreamReader;
use ringbuf::RingBuffer;

#[derive(Debug,PartialEq)]
enum State {
	Initialized,
	Playing,
	Finished,
}

//#[derive(Debug)]
pub struct MusicMiniaudio {
	stream_reader:	Option< OggStreamReader< Box< dyn FileLoaderFile > > >,
	consumer: 		ringbuf::Consumer< f32 >,
	producer: 		ringbuf::Producer< f32 >,
	state: 			State,
	largest_seen_packet:	usize,
}

impl MusicMiniaudio {

	pub fn new() -> Self {
		let mut rb = RingBuffer::new( 20*2944 /* 2*1024*1024 */ ); //size is randomly choosen
		let ( producer, consumer ) = rb.split();

		Self {
			stream_reader: None,
			consumer,
			producer,
			state: State::Initialized,
			largest_seen_packet: 0,
		}
	}
	pub fn load( &mut self, fileloader: &mut impl FileLoader, filename: &str ) -> bool {
		let mut f = fileloader.open( &filename );
		if !f.is_valid() {
			println!("Couldn't open file: {}", filename);
			return false;
		}

		dbg!( &f );
		let mut srr = match OggStreamReader::new(f) {
			Ok( srr ) => srr,
			Err( e ) => {
				eprintln!( "Error opening Ogg: {}, {:?}", &filename, &e);
				todo!("");
				return false;
			},
		};

		let sample_rate = srr.ident_hdr.audio_sample_rate as i32;

		eprintln!("Sample Rate: {}", sample_rate);
		if sample_rate != 48000 {
			panic!("Resampling not implemented please reencode with 48000 khz");
		}

		if srr.ident_hdr.audio_channels != 2 {
			panic!("Non stereo not implemented please reencode with 2 channels");
		}

		self.stream_reader = Some( srr );
		self.state = State::Playing;

		// prefill buffer
		for _ in 0..20 {
			if self.decode_packet() {
				break;
			}
		}
/*
		// pre decode max
		while !self.decode_packet() {

		}
*/		
		true
	}

	fn decode_packet( &mut self ) -> bool {
		if let Some( srr ) = &mut self.stream_reader {

			// theoretical max decoded size for packet: 1572864 ??
			if self.producer.remaining() < 10*2944 /* 1572864 */ {
				// do not decode if we are not sure it will fit
				eprintln!("Music buffer almost full {}/{}. Skipping decoding!", self.producer.len(), self.producer.capacity());
				return false;
			};

			match srr.read_dec_packet_itl() {
				Ok( pck_samples ) => {
					match pck_samples {
						Some( pck_samples ) => {
//							eprintln!("{}", pck_samples.len());
							let c = self.producer.remaining();

							if pck_samples.len() > self.largest_seen_packet {
								self.largest_seen_packet = pck_samples.len();
							}

							if c < pck_samples.len() {
								eprintln!("buffer full, stopping music decoding -> {}", self.producer.len());
								todo!("this would throw away data");
								return true;
							}

							let mut buffer: Vec< f32 > = Vec::with_capacity( pck_samples.len() );

							for p in 0..pck_samples.len() {
								let v = pck_samples[ p ];
								let v = ( v as f32 ) / 32767.0;
								buffer.push( v );
							};

							self.producer.push_slice( &buffer );

							return false;
						},
						None => {
							self.state = State::Finished;
							eprintln!("Finished decoding music.");
							eprintln!("largest_seen_packet: {}", self.largest_seen_packet);
							return true;
						}
					}
				},
				Err( e ) => {
					eprintln!("Error decode packet: {:?}", &e );
					return true;
				}
			}
		}
		true
	}

	pub fn play( &mut self ) {
	}

	pub fn pause( &mut self ) {
	}

	pub fn stop( &mut self ) {
	}

	pub fn update( &mut self, _time_step: f64 ) {

		for _ in 0..=4 {
			match self.state {
				State::Playing => {
					self.decode_packet();
				},
				_ => {},
			};
		}
	}


	pub fn fill_slice( &mut self, slice: &mut [f32] ) {

		eprintln!("Music buffer: {}", self.consumer.len());
		let l = slice.len();
		let mut c = 0;
		while c < l {
			if let Some( v ) = self.consumer.pop() {
				slice[ c ] += v;
				c += 1;
			} else {
				break;
			}
		}
		if c < l && self.state == State::Playing {
			println!("Starving music playback {} < {}", c, l );
		}
	}


}

