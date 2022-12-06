#[cfg(all(target_os = "macos", feature = "use_apple"))]
mod music_apple;
#[cfg(all(target_os = "macos", feature = "use_apple"))]
pub use music_apple::MusicApple as Music;

#[cfg(all(target_os = "macos", not(feature = "use_apple")))]
mod music_stub;
#[cfg(all(target_os = "macos", not(feature = "use_apple")))]
pub use music_stub::MusicStub as Music;

#[cfg(target_os = "windows")]
mod music_stub;
#[cfg(target_os = "windows")]
pub use music_stub::MusicStub as Music;

#[cfg(target_os = "linux")]
mod music_stub;
#[cfg(target_os = "linux")]
pub use music_stub::MusicStub as Music;
