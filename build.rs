use cfg_aliases::cfg_aliases;

pub fn main() {

	#[cfg(all(feature = "use_apple",feature = "use_miniaudio"))]
	{
		panic!("Backend apple and miniaudio are exclusive for the moment. Just choose one.");
	}

	#[cfg(not(any(feature = "use_apple",feature = "use_miniaudio")))]
	{
//		panic!("Warning no backend selected. Stub would be used.");
		// :TODO: emit warning
	}

	#[cfg(all(feature = "use_apple", not(target_os = "macos" )))]
	{
		panic!("Backend apple only works on macOS. (iOS, ipadOS, tvOS coming soon)");
	}

	cfg_aliases! {
		macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        windows: { target_os = "windows" },

        // backends
        use_apple: { all(macos, feature = "use_apple", not(wasm)) },
        use_miniaudio: { all(feature = "use_miniaudio", not(wasm)) },
        use_stub: { not(any(feature = "use_miniaudio", feature = "use_apple" ) ) }, // blocked by panic above
	}

    if std::env::var("TARGET").unwrap().contains("-apple") {
		if cfg!( feature = "use_apple" ) {
	        println!("cargo:rustc-link-lib=framework=Foundation");
	        println!("cargo:rustc-link-lib=framework=AVFAudio");
	    }
    }

}
