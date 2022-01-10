
#[cfg(all(target_os = "macos", feature = "apple"))]
mod audio_apple;
#[cfg(all(target_os = "macos", feature = "apple"))]
pub use audio_apple::AudioApple as Audio;

#[cfg(all(target_os = "macos", not( feature = "apple" ) ))]
mod audio_miniaudio;
#[cfg(all(target_os = "macos", not( feature = "apple" ) ))]
pub use audio_miniaudio::AudioMiniaudio as Audio;

#[cfg(target_os = "windows")]
mod audio_stub;
#[cfg(target_os = "windows")]
pub use audio_stub::AudioStub as Audio;

#[cfg(target_os = "linux")]
mod audio_stub;
#[cfg(target_os = "linux")]
pub use audio_stub::AudioStub as Audio;
