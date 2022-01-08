# Fourtris

This is a very simplified implementation of Tetris. The purpose is not to be the most faithful reproduction of Tetris, rather to (re)familiarize myself with rust.

This is a library crate along with an example implementation. The example is also very simple - no start screen, no pause, no restart, no game over screen.

There will be bugs. Let me know what you find.

## Controls
Left Arrow - Move a piece to the left.
Right Arrow - Move a piece ot the right.
Down Arrow - Hold down to make a piece fall faster. (No hard drop implemented)
Q - Counterclockwise rotation.
W - Clockwise rotation.

## Dependencies

If you want to try the example, you will need SDL2 installed.

On Ubuntu based distros you can install it the following way.

```
sudo apt-get install libsdl2-dev libsdl2-ttf-dev
```

You will also need rust installed. The version I used is: `rustc 1.58.0-nightly (8b09ba6a5 2021-11-09)`.


## Running

To run the example program, type:

`cargo run --features=full_redraw --example sdl2backend`


Enjoy!
