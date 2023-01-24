#[cfg(use_apple)]
mod audio_apple;
#[cfg(use_apple)]
pub use audio_apple::AudioApple;

#[cfg(use_miniaudio)]
mod audio_miniaudio;
#[cfg(use_miniaudio)]
pub use audio_miniaudio::AudioMiniaudio;

#[cfg(use_stub)]
mod audio_stub;
#[cfg(use_stub)]
pub use audio_stub::AudioStub;

//use core::cell::OnceCell;
use crate::AudioBackend;

pub struct Audio<F> {
	//x: PhantomData<F>,
	_marker: std::marker::PhantomData<F>,
}

//static BACKENDS: OnceCell<Vec<(String,dyn Fn() -> ())>> = OnceCell::new();
/*
static BACKENDS: Vec<(String,&dyn Fn() -> dyn AudioBackend)> = [
	("apple",AudioApple::new),
];
*/

impl<F: crate::FileLoader> Audio<F> {
	#[deprecated(since = "0.6.10", note = "Please use Audio::create_default()")]
	pub fn new() -> Box<dyn AudioBackend<F>> {
		Audio::create_default()
	}
	#[allow(unreachable_code)]
	pub fn create_default() -> Box<dyn AudioBackend<F>> {
		#[cfg(use_apple)]
		{
			return Box::new(AudioApple::new());
		}
		#[cfg(use_miniaudio)]
		{
			return Box::new(AudioMiniaudio::new());
		}
		#[cfg(use_stub)]
		{
			return Box::new(AudioStub::new());
		}
	}

	pub fn create(backend: &str) -> Box<dyn AudioBackend<F>> {
		let be: Box<dyn AudioBackend<F>> = match backend {
			#[cfg(use_apple)]
			"apple" => Box::new(AudioApple::new()),
			#[cfg(use_miniaudio)]
			"miniaudio" => Box::new(AudioMiniaudio::new()),
			#[cfg(use_stub)]
			_ => Box::new(AudioStub::new()),
			#[cfg(not(use_stub))]
			_ => panic!("backend not found, and stub not available"),
		};

		be
	}
}
