# neovim-mpc

## Installation

### [vim-plug](https://github.com/junegunn/vim-plug)

Add to your plugin block:

```vim
Plug 'lfkeitel/neovim-mpc', { 'do': 'bash install.sh' }
```

Then run `:PlugInstall`.

## Usage

Refer to the following table to find supported commands.

| Command  | Description |
|----------|-------------|
| `:MpcCurrentSong` | Echo the current song playing |
| `:MpcNext` | Play next track |
| `:MpcPrevious` | Play previous track |
| `:MpcPlay` | Play the track, if currently paused |
| `:MpcPause` | Pause the track, if currently playing |
| `:MpcToggle` | Toggle between play and pause |

## TODOs

* Better error handling - get rid of `unwrap()`s everywhere.

## License

BSD 3-clause

## Original

Inspired by [neovim-spotify](https://github.com/srishanbhattarai/neovim-spotify).
The structure is the same but refactored to work with mpc.

Original author's blog post: https://medium.com/@srishanbhattarai/a-detailed-guide-to-writing-your-first-neovim-plugin-in-rust-a81604c606b1.
