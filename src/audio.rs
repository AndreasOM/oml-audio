
#[cfg(use_apple)]
	mod audio_apple;
#[cfg(use_apple)]
	pub use audio_apple::AudioApple as Audio;

#[cfg(use_miniaudio)]
	mod audio_miniaudio;
#[cfg(use_miniaudio)]
	pub use audio_miniaudio::AudioMiniaudio as Audio;

#[cfg(use_stub)]
	mod audio_stub;
#[cfg(use_stub)]
	pub use audio_stub::AudioStub as Audio;
