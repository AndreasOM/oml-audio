pub fn main() {

    if std::env::var("TARGET").unwrap().contains("-apple") {
		//#[cfg(all(target_os = "macos", feature = "apple"))]
		if cfg!( feature = "apple" ) {
	        println!("cargo:rustc-link-lib=framework=Foundation");
	        println!("cargo:rustc-link-lib=framework=AVFAudio");
	    }
    }

}