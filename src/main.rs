#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    prelude::*, system::SystemControl, peripherals::Peripherals,
    clock::ClockControl,
    gpio::Io,
    uart::Uart,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut uart1 = Uart::new(peripherals.UART1, &clocks, io.pins.gpio1, io.pins.gpio3).unwrap();
    uart1.write_bytes(b" Hello world!\n").expect("This should work");

    loop {
    }
}
