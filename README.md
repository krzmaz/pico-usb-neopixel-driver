# pico-usb-neopixel-driver
Simple way to drive a lot of neopixels *fast* via USB

The protocol is as dumb as it gets: send length of the data to be displayed on two bytes, than send the data for each pixel

# Runners
The default runner is [picotool](https://github.com/raspberrypi/picotool) and you can get it via `brew install picotool`, or compile it yourself. The benefit of using it is that thanks to [usbd-picotool-reset](https://github.com/ithinuel/usbd-picotool-reset) it is able to reboot the pico into BOOTSEL mode, so you don't have to worry about it.

Alternatively, you can use `elf2uf2-rs` by switching to that runner in `.cargo/config.toml`, after installing it via
```
cargo install elf2uf2-rs --locked
```

# Examples used for this repo:
- https://github.com/rp-rs/rp2040-project-template
- https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_ws2812_led.rs