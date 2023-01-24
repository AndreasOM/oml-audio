#[cfg(use_apple)]
mod music_apple;
#[cfg(use_apple)]
pub use music_apple::MusicApple; // as Music;

#[cfg(use_miniaudio)]
mod music_miniaudio;
#[cfg(use_miniaudio)]
pub use music_miniaudio::MusicMiniaudio; // as Music;

#[cfg(use_stub)]
mod music_stub;
#[cfg(use_stub)]
pub use music_stub::MusicStub; // as Music;
