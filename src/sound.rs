
#[cfg(all(target_os = "macos", feature = "apple"))]
mod sound_apple;
#[cfg(all(target_os = "macos", feature = "apple"))]
pub use sound_apple::SoundPoolApple as SoundPool;

#[cfg(all(target_os = "macos", not( feature = "apple" ) ))]
mod sound_stub;
#[cfg(all(target_os = "macos", not( feature = "apple" ) ))]
pub use sound_stub::SoundPoolStub as SoundPool;

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
