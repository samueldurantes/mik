#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

pub mod io;
pub mod macros;
pub mod time;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Setup
    let pin = 8u8;
    io::config_pin_output(pin);

    loop {
        unsafe {
            io::pin_clear(pin);
            time::delay_ms(250);
            io::pin_set(pin);
            time::delay_ms(250);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
