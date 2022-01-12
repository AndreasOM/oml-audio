
use std::cell::RefCell;
use std::sync::Arc;

use crate::FileLoader;
use crate::{
	WavFile,
	WavPlayer,
};

use miniaudio::{Device, DeviceConfig, DeviceType, Format};
use miniaudio::{Waveform, WaveformConfig, WaveformType};
use miniaudio::{FramesMut};

use ringbuf::RingBuffer;

pub type DeviceFormatType = f32;
pub const DEVICE_FORMAT: Format = Format::F32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = miniaudio::SAMPLE_RATE_48000;

use miniaudio::Context;


struct Synth {
	phase: f32,
	freq: f32,
}

impl Synth {
	pub fn new( freq: f32 ) -> Self {
		Self {
			phase: 0.0,
			freq,
		}
	}

	pub fn next_sample( &mut self ) -> f32 {
		let s = self.phase;

		// freq = period per second
		// DEVICE_SAMPLE_RATE = samples per second
		// period = a cycle of 0.0 -> 1.0
		// 
		self.phase += self.freq/ ( DEVICE_SAMPLE_RATE as f32 ); //0.1 * self.freq;
		if self.phase > 1.0 {
			self.phase -= 1.0;
		}

		s
	}
}

struct Buffer {
	consumer: ringbuf::Consumer< f32 >
}

impl Buffer {
	pub fn new( consumer: ringbuf::Consumer< f32 > ) -> Self {
		Self {
			consumer,
		}
	}

	pub fn data_output_callback(&mut self, output: &mut FramesMut) -> u64 {
		let mut samples = output.as_samples_mut::<f32>();
		let l = samples.len();
		let mut c = 0;
		while c < l {
			if let Some( v ) = self.consumer.pop() {
				samples[ c ] = v;
				c += 1;
			} else {
				break;
			}
		}
		if c < l {
			println!("Starving {} < {}", c, l );
		}
//		dbg!( c, l );
		0
	}
}

pub struct AudioMiniaudio {
	device:		Device,
	producer:	ringbuf::Producer< f32 >,
//	wave:		Waveform,
	synth:		Synth,
	wav_file:	WavFile,
	wav_player: WavPlayer,
	capture_size: usize,
	capture_count: usize,
	capture_buffer: Vec< f32 >,
}

impl AudioMiniaudio {
	pub fn new() -> Self {
		/*
		let sine_wave_config = WaveformConfig::new(
		    DEVICE_FORMAT,
		    DEVICE_CHANNELS,
		    DEVICE_SAMPLE_RATE,
		    WaveformType::Sine,
		    0.2,
		    220.0,
		);
		let mut sine_wave = Waveform::new(&sine_wave_config);
		*/
		let mut device_config = DeviceConfig::new(DeviceType::Playback);
		device_config.playback_mut().set_format(DEVICE_FORMAT);
		device_config.playback_mut().set_channels(DEVICE_CHANNELS);
		device_config.set_sample_rate(DEVICE_SAMPLE_RATE);

		let mut rb = RingBuffer::new( 4*4096 );
		let ( producer, consumer ) = rb.split();
		let mut buffer = Buffer::new( consumer );

/*
		device_config.set_data_callback(move |_device, output, _input| {
			buffer.data_output_callback( output );
		});
*/

		device_config.set_stop_callback(|_device| {
		    println!("Device Stopped.");
		});

		let mut device = Device::new(None, &device_config).expect("failed to open playback device");
		device.set_data_callback(move |_device, output, _input| {
			buffer.data_output_callback( output );
		});
		device.start().expect("failed to start device");

		println!("Device Backend: {:?}", device.context().backend());

		Self {
			device,
			producer,
//			wave: sine_wave,
			synth:	Synth::new( 440.0 ),
			wav_file: WavFile::new(),
			wav_player: WavPlayer::new(),
			capture_size: 0,
			capture_count: 0,
			capture_buffer: Vec::new(),
		}
	}

	pub fn update( &mut self, timestep: f64 ) {
//		self.music.update( timestep );
//		self.sound_bank.update( timestep );
		// create temp holder to get data out of waveform
/*
pub fn wrap<S: Sample>(
    data: &'s mut [S],
    format: Format,
    channels: u32
) -> FramesMut<'s>
*/
//		let l = self.producer.remaining();
/*
		let mut data = [0f32;4096];

		let mut frames = FramesMut::wrap( &mut data, DEVICE_FORMAT, DEVICE_CHANNELS );

		let l = self.wave.read_pcm_frames( &mut frames ) as usize;
//		dbg!(l);

		if self.producer.remaining() < l {
			println!("throwing away data");
		}
*/
/*
		let mut c = 0;
		while self.producer.remaining() > 0 { // && c < l {
//			let v = data[ c ];
			let v = self.synth.next_sample();
//			print!("{} ", v);
			self.producer.push( v );
			self.producer.push( v );
			c += 1;
		}
*/
		let mut c = 0;
		while self.producer.remaining() > 0 { // && c < l {
//			let v = data[ c ];
			let ( l, r ) = if self.wav_player.done() {
				( 0.0, 0.0 )
			} else {
				let l = self.wav_player.next_sample( &self.wav_file );
				let r = self.wav_player.next_sample( &self.wav_file );
				( l, r )
			};
			self.producer.push( l );
			self.producer.push( r );
			if self.capture_count < self.capture_size {
				self.capture_buffer.push( l );
				self.capture_count += 1;
			}
			c += 1;
		}
	}

	pub fn load_music( &mut self, fileloader: &mut impl FileLoader, filename: &str ) -> bool {
//		self.music.load( fileloader, filename )
		true
	}

	pub fn play_music( &mut self ) {
//		self.music.play();
	}

	pub fn load_sound_bank( &mut self, fileloader: &mut impl FileLoader, filename: &str ) {
//		self.sound_bank.load( fileloader, filename )
//		self.wav_file.load( fileloader, "coin48000.wav" );
//		self.wav_file.load( fileloader, "sine440hz48000.wav" );
		self.wav_file.load( fileloader, "music.wav" );
//		dbg!(&self.wav_file);
//		todo!("die");
	}

	pub fn play_sound( &mut self, name: &str ) {
//		self.sound_bank.play( name );
		self.wav_player.play();
	}

	pub fn capture( &mut self, size: usize ) {
		self.capture_buffer.clear();
		self.capture_buffer.reserve_exact( size );
		self.capture_size = size;
		self.capture_count = 0;
	}

	pub fn capture_buffer_slice( &self ) -> &[f32] {
		self.capture_buffer.as_slice()
	}

	pub fn list_devices( &self ) {
	    let context = Context::new(&[], None).expect("failed to create context");

	    context
	        .with_devices(|playback_devices, capture_devices| {
	            println!("Playback Devices:");
	            for (idx, device) in playback_devices.iter().enumerate() {
	                println!("\t{}: {}", idx, device.name());
	            }

	            println!("Capture Devices:");
	            for (idx, device) in capture_devices.iter().enumerate() {
	                println!("\t{}: {}", idx, device.name());
	            }
	        })
	        .expect("failed to get devices");		
	}
}
