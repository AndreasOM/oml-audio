# TODO

## In Progress


## TODO

- [ ] Allow runtime selection of backend
- [ ] Add fading over time
- [ ] Use tracing for output
- [ ] Reduce debug noise
- [ ] Implement `pause_music` for miniaudio.
- [ ] Add better setup error information in debug mode, e.g. `update` called before `start`.
- [ ] Expose buffer size tuning to user

- [ ] Implement capturing for apple
- [ ] Ensure cross compile to linux and windows works

- [ ] Split sound and sound pool logic

- [ ] Run post push github action to catch errors
- [ ] Publish package via github action


## DONE

- [x] Play music (.ogg) with miniaudio
- [x] Implement SoundBank for miniaudio (might just need SoundPoolMiniaudio)
- [x] Add example to README
- [x] Use feature flag to enable apple specific implementation
- [x] Load sounds via sound bank
- [x] Fix sound reuse bug
- [x] Add native music player for macOS
- [x] Add native sound player for macOS

## Released
