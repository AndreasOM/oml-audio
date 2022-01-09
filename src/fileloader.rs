use std::fs::{ File, OpenOptions };
use std::io::{BufReader,SeekFrom};
use std::io::prelude::*;


pub trait FileLoaderFile {
	fn is_valid( &self ) -> bool;
	fn read_u8( &mut self ) -> u8;
	fn eof( &self ) -> bool;
	fn name( &self ) -> &str;

	fn read_u16( &mut self ) -> u16 {
		let a = self.read_u8() as u16;
		let b = self.read_u8() as u16;

		  ( b << 8 )
		| ( a << 0 )
	}

	fn read_u32( &mut self ) -> u32 {
		let a = self.read_u8() as u32;
		let b = self.read_u8() as u32;
		let c = self.read_u8() as u32;
		let d = self.read_u8() as u32;

		  ( d << 24 )
		| ( c << 16 )
		| ( b <<  8 )
		| ( a <<  0 )
	}
}


pub trait FileLoader {
	fn open( &mut self, filename: &str ) -> Box< dyn FileLoaderFile >;
	fn exists( &self, filename: &str ) -> bool;
}


pub struct FileLoaderFileDisk {
	filename: String,	// only needed for better debugging
	file: Option< BufReader< File > >,
	size: usize,
	pos: usize,
}

impl FileLoaderFileDisk {
	pub fn open( filename: &str ) -> Self {
		let mut s = Self {
			filename: filename.to_string(),
			file: None,
			size: 0,
			pos: 0,
		};

		if let Ok( mut f ) = File::open( &s.filename ) {
			if let Ok( p ) =  f.seek(SeekFrom::End(0)) {
				f.seek( SeekFrom::Start( 0 ) ).unwrap();
				s.size = p as usize
			} else {
			}
			let f = BufReader::new(f);

			s.file = Some( f );
		};

		s
	}
}

impl FileLoaderFile for FileLoaderFileDisk {
	fn read_u8( &mut self ) -> u8 {

		match &mut self.file {
			Some( f ) => {
				let mut buf = [0];
				match f.read( &mut buf ) {
					Ok( _ ) => {
						self.pos += 1;
						buf[ 0 ]
					},
					Err( _ ) => 0,
				}

			},
			None => {
				0
			},
		}

	}
	fn is_valid( &self ) -> bool {
		self.file.is_some()
	}
	fn eof( &self ) -> bool {
		self.pos >= self.size
	}
	fn name( &self ) -> &str {
		&self.filename
	}	
}

pub struct FileLoaderDisk {
	basedir:	String,
	debug:		bool,
}

impl FileLoaderDisk {
	pub fn new( basedir: &str ) -> Self {
		Self {
			basedir: 	basedir.to_string(),
			debug:		false,
		}
	}
	pub fn enable_debug ( &mut self ) {
		self.debug = true;
	}
}

impl FileLoader for FileLoaderDisk {
	fn open( &mut self, filename: &str ) -> Box< dyn FileLoaderFile > {
		let fullname = format!("{}/{}", &self.basedir, &filename);
		if self.debug { println!("FileLoaderDisk opening {}", fullname); }
		let stream = FileLoaderFileDisk::open( &fullname );

		Box::new( stream )
	}
	fn exists( &self, filename: &str ) -> bool {
		let fullname = format!("{}/{}", &self.basedir, &filename);
		std::path::Path::new(&fullname).exists()
	}


}
