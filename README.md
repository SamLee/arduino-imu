# arduino-imu

Using the BMI160 with an arduino uno.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`,
   `avr-libc`, `avrdude`, [`ravedude`]).
2. Run `cargo build` to build the firmware.
3. Run `cargo run` to flash the firmware to a connected board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Notes for windows
If using wsl use `usbipd` to forward the device to wsl.

Run `usbipd list` to get the busid of the device.

Run `usbipd wsl attach --busid 5-2` to attach the device.
