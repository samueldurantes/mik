#![no_std]
#![no_main]

use mik::io;

const BUTTON_PIN: u8 = 10;
const LED_PIN: u8 = 8;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    io::config_pin_input_pullup(BUTTON_PIN);
    io::config_pin_output(LED_PIN);

    loop {
        if io::pin_read(BUTTON_PIN) == 0 {
            io::pin_set(LED_PIN);
        } else {
            io::pin_clear(LED_PIN);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
