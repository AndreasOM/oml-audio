
#[cfg(all(target_os = "macos", feature = "use_apple"))]
mod sound_apple;
#[cfg(all(target_os = "macos", feature = "use_apple"))]
pub use sound_apple::SoundPoolApple as SoundPool;

#[cfg(all(target_os = "macos", not( feature = "use_apple" ) ))]
mod sound_miniaudio;
#[cfg(all(target_os = "macos", not( feature = "use_apple" ) ))]
pub use sound_miniaudio::SoundPoolMiniaudio as SoundPool;

#[cfg(target_os = "windows")]
mod sound_stub;
#[cfg(target_os = "windows")]
pub use sound_stub::SoundPoolStub as SoundPool;

#[cfg(target_os = "linux")]
mod sound_stub;
#[cfg(target_os = "linux")]
pub use sound_stub::SoundPoolStub as SoundPool;


mod sound_bank;
pub use sound_bank::SoundBank as SoundBank;

mod drop_mode;
pub use drop_mode::DropMode;
