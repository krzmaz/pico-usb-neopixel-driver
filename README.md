# pico-usb-neopixel-driver
Simple way to drive a lot of neopixels *fast* via USB

The protocol is as dumb as it gets: send length of the data to be displayed on two bytes, than send the data for each pixel

# Building
Simplest build:
```
./build.sh
```
## Enabling debug logs

**Note** The command will fail, but CMake will pick up the change, you need to run `build.sh` again after the change to rebuild
```
./build.sh -DDEBUG_LOGS=1
# or
./build.sh -DDEBUG_LOGS=0
```

# Running
Just drop the uf2 file to the Pico booted in BOOTSEL mode:
```
cp build/src/pico_usb_neopixel_driver.uf2 /Volumes/RPI-RP2
```