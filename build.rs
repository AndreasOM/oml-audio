pub fn main() {

    if std::env::var("TARGET").unwrap().contains("-apple") {
		if cfg!( feature = "use_apple" ) {
	        println!("cargo:rustc-link-lib=framework=Foundation");
	        println!("cargo:rustc-link-lib=framework=AVFAudio");
	    }
    }

}