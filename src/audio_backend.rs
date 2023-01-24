use crate::FileLoader;

pub trait AudioBackend<F>
where
	F: FileLoader,
{
	fn start(&mut self) {}
	fn load_sound_bank(&mut self, _fileloader: &mut F, _filename: &str);
	fn update(&mut self) -> f64;
	fn play_music(&mut self);
	fn play_sound(&mut self, _name: &str);
	fn load_music_native(&mut self, fileloader: &mut F, filename: &str) -> bool;
}

impl<F> core::fmt::Debug for dyn AudioBackend<F> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "[dyn AudioBackend]")
	}
}
