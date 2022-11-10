# suletta-fx

Plugins to accompany the `suletta` synthesizer project.

## Effects Available

### Gain

Simple plugin to change the gain at any point in your audio chain.

### Saturation

Simple `tanh()` based saturation plugin. Comfortingly familiar distortion.

### Filter

wip

### Compressor

wip

### Delay

wip

### Reverb

wip

## Building

Clone the repository, then run:

`cargo xtask bundle <plugin> --release`

replacing `<plugin>` with the name of the plugin you wish to use.

## Todo

- [ ] Basic functioning suite
    - [x] Simplistic gain fx plugin
    - [x] Saturation
        - [ ] Different flavors?
    - [ ] Compression
    - [ ] Delay
    - [ ] Reverb
    - [ ] Filter
