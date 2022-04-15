
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
	
/*
#[cfg(all(target_os = "macos", feature = "use_apple"))]
mod audio_apple;
#[cfg(all(target_os = "macos", feature = "use_apple"))]
pub use audio_apple::AudioApple as Audio;

#[cfg(all(target_os = "macos", feature = "use_miniaudio"))]
mod audio_miniaudio;
#[cfg(all(target_os = "macos", feature = "use_miniaudio"))]
pub use audio_miniaudio::AudioMiniaudio as Audio;

#[cfg(all(target_os = "macos", not( any( feature = "use_apple", feature= "use_miniaudio"  ) ) ) )]
mod audio_stub;
#[cfg(all(target_os = "macos", not( any( feature = "use_apple", feature= "use_miniaudio"  ) ) ) )]
pub use audio_stub::AudioStub as Audio;


#[cfg(all(target_os = "windows", not( any( feature= "use_miniaudio"  ) ) ) )]
mod audio_stub;
#[cfg(all(target_os = "windows", not( any( feature= "use_miniaudio"  ) ) ) )]
pub use audio_stub::AudioStub as Audio;

#[cfg(all(target_os = "windows", feature = "use_miniaudio"))]
mod audio_miniaudio;
#[cfg(all(target_os = "windows", feature = "use_miniaudio"))]
pub use audio_miniaudio::AudioMiniaudio as Audio;

#[cfg(target_os = "linux")]
mod audio_stub;
#[cfg(target_os = "linux")]
pub use audio_stub::AudioStub as Audio;
*/

