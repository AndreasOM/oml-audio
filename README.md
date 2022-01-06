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
