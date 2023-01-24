use miniaudio::FramesMut;
use miniaudio::{Device, DeviceConfig, DeviceType, Format};
use ringbuf::RingBuffer;

use crate::music::MusicMiniaudio;
use crate::AudioBackend;
use crate::FileLoader;
use crate::SoundBank;

//pub type DeviceFormatType = f32;
pub const DEVICE_FORMAT: Format = Format::F32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = miniaudio::SAMPLE_RATE_48000;

use std::time::Instant;

use miniaudio::Context; // temporary, we get higher precision by calculating from the audio callbacks

struct Buffer {
	consumer: ringbuf::Consumer<f32>,
}

impl Buffer {
	pub fn new(consumer: ringbuf::Consumer<f32>) -> Self {
		Self { consumer }
	}

	pub fn data_output_callback(&mut self, output: &mut FramesMut) -> u64 {
		let mut samples = output.as_samples_mut::<f32>();
		let l = samples.len();
		let mut c = 0;
		while c < l {
			if let Some(v) = self.consumer.pop() {
				samples[c] = v;
				c += 1;
			} else {
				break;
			}
		}
		if c < l {
			println!("Starving {} < {}", c, l);
		}
		//		dbg!( c, l );
		0
	}
}

pub struct AudioMiniaudio {
	device:         Option<Device>,
	producer:       Option<ringbuf::Producer<f32>>,
	last_now:       Instant,
	sound_bank:     SoundBank,
	music:          MusicMiniaudio,
	//	wave:			Waveform,
	//	wav_file:		WavFile,
	//	wav_player:		WavPlayer,
	capture_size:   usize,
	capture_count:  usize,
	capture_buffer: Vec<f32>,
}

impl std::fmt::Debug for AudioMiniaudio {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AudioMiniaudio")
			.field("sound_bank", &self.sound_bank)
			.finish()
	}
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

		/*
				device_config.set_data_callback(move |_device, output, _input| {
					buffer.data_output_callback( output );
				});
		*/

		Self {
			device:         None,
			producer:       None,
			last_now:       Instant::now(),
			sound_bank:     SoundBank::new(),
			music:          MusicMiniaudio::new(),
			//			wave: sine_wave,
			//			wav_file: WavFile::new(),
			//			wav_player: WavPlayer::new(),
			capture_size:   0,
			capture_count:  0,
			capture_buffer: Vec::new(),
		}
	}

	pub fn get_sound_bank_mut(&mut self) -> &mut SoundBank {
		&mut self.sound_bank
	}

	pub fn fill_buffer(
		sound_bank: &mut SoundBank,
		music: &mut MusicMiniaudio,
		producer: &mut ringbuf::Producer<f32>,
	) -> usize {
		let c = producer.remaining();

		let mut buffer = Vec::with_capacity(c);
		for _ in 0..c {
			buffer.push(0.0);
		}

		let mut buffer = buffer.as_mut_slice();

		sound_bank.fill_slice(&mut buffer);

		music.fill_slice(&mut buffer);

		producer.push_slice(buffer);

		dbg!(c);
		c
	}

	pub fn drain_buffer(consumer: &mut ringbuf::Consumer<f32>) -> usize {
		let mut c = 0;
		while let Some(v) = consumer.pop() {
			c += 1;
		}

		c
	}

	pub fn load_music(&mut self, fileloader: &mut impl FileLoader, filename: &str) -> bool {
		self.music.load(fileloader, filename)
	}


	pub fn capture(&mut self, size: usize) {
		self.capture_buffer.clear();
		self.capture_buffer.reserve_exact(size);
		self.capture_size = size;
		self.capture_count = 0;
	}

	pub fn capture_buffer_slice(&self) -> &[f32] {
		self.capture_buffer.as_slice()
	}

	pub fn list_devices(&self) {
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

impl<F: crate::FileLoader> AudioBackend<F> for AudioMiniaudio {
	fn start(&mut self) {
		let mut device_config = DeviceConfig::new(DeviceType::Playback);
		device_config.playback_mut().set_format(DEVICE_FORMAT);
		device_config.playback_mut().set_channels(DEVICE_CHANNELS);
		device_config.set_sample_rate(DEVICE_SAMPLE_RATE);

		let mut rb = RingBuffer::new(4 * 4096);
		let (producer, consumer) = rb.split();
		let mut buffer = Buffer::new(consumer);

		device_config.set_stop_callback(|_device| {
			println!("Device Stopped.");
		});

		let mut device = Device::new(None, &device_config).expect("failed to open playback device");
		device.set_data_callback(move |_device, output, _input| {
			buffer.data_output_callback(output);
		});
		device.start().expect("failed to start device");

		println!("Device Backend: {:?}", device.context().backend());

		self.device = Some(device);
		self.producer = Some(producer);
	}

	fn update(&mut self) -> f64 {
		let timestep = self.last_now.elapsed().as_secs_f64();
		self.last_now = Instant::now();

		self.music.update(timestep);
		self.sound_bank.update(timestep);

		if let Some(producer) = &mut self.producer {
			AudioMiniaudio::fill_buffer(&mut self.sound_bank, &mut self.music, producer);
		}

		timestep
	}
	fn load_sound_bank(&mut self, fileloader: &mut F, filename: &str) {
		self.sound_bank.load(fileloader, filename);
	}

	fn play_music(&mut self) {
		self.music.play();
	}
	fn pause_music(&mut self) {
		self.music.pause();
	}

	fn is_music_playing(&self) -> bool {
		false
	}


	fn play_sound(&mut self, name: &str) {
		println!("Playing {}", &name);
		self.sound_bank.play(name);
	}

	fn is_any_sound_playing(&self) -> bool {
		self.sound_bank.is_any_sound_playing()
	}
	
	fn load_music_native(&mut self, fileloader: &mut F, filename: &str) -> bool {
		let filename = format!("{}.ogg", filename);
		self.music.load(fileloader, &filename)
	}
}
