#![feature(asm_experimental_arch)]
#![no_std]

use core::arch::asm;

pub mod io;
pub mod macros;

pub unsafe fn delay_ms(mut ms: u16) {
    while ms != 0 {
        unsafe {
            asm!(
                "2:",
                "sbiw {0}, 1",
                "brne 2b",
                inout(reg_iw) 4000u16 => _, // This is a 16-bit register that stores `4000` value.
            );
        }

        ms -= 1;
    }
}
