#![no_std]
#![no_main]

mod binary_info;
mod panic;

use heapless::Vec;
use itertools::Itertools;

// The macro for our start-up function
use rp_pico::entry;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// Import the Timer for Ws2812:
use rp_pico::hal::timer::Timer;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB PicoTool Class Device support
use usbd_picotool_reset::PicoToolReset;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// PIOExt for the split() method that is needed to bring
// PIO0 into useable form for Ws2812:
use rp_pico::hal::pio::PIOExt;

// Import useful traits to handle the ws2812 LEDs:
use smart_leds::SmartLedsWrite;

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Set up the USB PicoTool Class Device driver
    let mut picotool: PicoToolReset<_> = PicoToolReset::new(&usb_bus);

    // Create a USB device RPI Vendor ID and on of these Product ID:
    // https://github.com/raspberrypi/picotool/blob/master/picoboot_connection/picoboot_connection.c#L23-L27
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x2e8a, 0x000a))
        .manufacturer("krzmaz")
        .product("Neopixel Driver")
        .serial_number("PICO")
        .device_class(0) // from: https://www.usb.org/defined-class-codes
        .build();

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Create a count down timer for the Ws2812 instance:
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // Split the PIO state machine 0 into individual objects, so that
    // Ws2812 can use it:
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Instantiate a Ws2812 LED strip:
    let mut ws = Ws2812::new(
        // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
        // for the LED data output:
        pins.gpio2.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    // 128 is the default for usb implementation
    let mut read_buf = [0u8; 128];
    let mut buf = Vec::<u8, 5000>::new();

    loop {
        // Check for new data
        if usb_dev.poll(&mut [&mut serial, &mut picotool]) {
            match serial.read(&mut read_buf) {
                Ok(count) if count > 0 => {
                    buf.extend(read_buf.iter().take(count).copied());
                    if buf.len() > 1 {
                        let length: u16 = u16::from_le_bytes([buf[0], buf[1]]);
                        if buf.len() >= (length + 2) as usize {
                            // there might be the start of the next "frame" in the input buffer
                            let overflow = buf.len() - (length as usize + 2);
                            let _ = ws.write(
                                buf.iter()
                                    .skip(2)
                                    .take(length as usize)
                                    .copied()
                                    .tuples::<(_, _, _)>(),
                            );
                            buf.clear();
                            if let Some(remainder) = read_buf.get((count - overflow)..overflow) {
                                // safe to unwrap since we clear the buffer above, so the remainder
                                // cannot extend its capacity
                                buf.extend_from_slice(remainder).unwrap();
                            }
                        }
                    }
                }
                _ => {
                    // Received error or zero bytes, ignore
                }
            }
        }
    }
}
