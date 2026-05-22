#![no_std]
#![no_main]

use mik::{delay_ms, io};

const LED_PIN: u8 = 8;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    io::config_pin_output(LED_PIN);

    loop {
        io::pin_toggle(LED_PIN);
        delay_ms(250);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
