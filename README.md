# oml-audio

Work in progress!
API will change, but we'll obey semver rules.
To be precise:

Pre 1.0.0
- breaking changes will change the minor version (x.*y*.0)
- non-breaking changes will change the patch level (x.y.*z*)

Post 1.0.0
- breaking changes will change the major version (*x*.0.0)
- non-breaking changes, additions will change the minor version (x.*y*.0)
- fixes will change the patch level (x.y.*z*)

0.y.z will potentially change a lot!

Very thin wrapper for playing music, and audio in rust based games.

Features:
- [x] Play .mp3 music (with hardware acceleration) on macOS
- [x] Play .wav & .caf sound on macOS

- [ ] Play .ogg music on windows
- [ ] Play .wav sound on windows
- [ ] Play .ogg music on linux
- [ ] Play .wav sound on linux


Future:
- [ ] Support other platforms (iOS, android, etc)

## Breaking changes in 0.8.x

Newer versions allow compiling in multiple backends and selecting them at runtime.
This has the downside of a breaking API change :(
`Audio::new`, or rather `Audio::create_default` or `Audio::create` now return a `Box<dyn AudioBackend>`
which might mean you have to change your storage.
AudioBackend is also now generic over `FileLoader`.

## Example :WIP:

```rust
use oml_audio::Audio;
use oml_audio::fileloader::FileLoaderDisk;

pub fn main() {
    let mut fileloader = FileLoaderDisk::new( "./data" ); // 'data' is the base directory for all other files/paths
    fileloader.enable_debug();

    // OLD: let mut audio = Audio::new();
    let mut audio: Box<dyn AudioBackend<FileLoaderDisk>> = Audio::create_default(); // explicit type for verbosity only
    audio.load_sound_bank( &mut fileloader, "test.omsb" );

    audio.play_sound( "SOUND_ID" );

    loop {
        let _timestep = audio.update();

        // update
        // render
        // (maybe) yield/sleep
    }

}
```
