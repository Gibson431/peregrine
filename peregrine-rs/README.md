# peregrine-rs #

This is an example of how to program the Peregrine flight computer using rust.

It uses the [embassy-rs](https://github.com/embassy-rs/embassy) framework to handle the async functionality.

It also introduces the concept of [embedded-hal](https://github.com/rust-embedded/embedded-hal) for creating generic sensor drivers (the BMP280 in this case).

## Usage

To use this crate, ensure you have rust installed properly with the rp2040 target archetecture.

'''bash
cargo run --release
'''

This should program the board and start a serial monitor.
