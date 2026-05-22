pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PORTC: *mut u8 = 0x28 as *mut u8;
pub const PORTD: *mut u8 = 0x2B as *mut u8;

pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const DDRC: *mut u8 = 0x29 as *mut u8;
pub const DDRD: *mut u8 = 0x2A as *mut u8;

pub const PINB: *mut u8 = 0x23 as *mut u8;
pub const PINC: *mut u8 = 0x26 as *mut u8;
pub const PIND: *mut u8 = 0x09 as *mut u8;

const BIT_MAP: [u8; 20] = [
    // PORTD
    0, // 0
    1, // 1
    2, // 2
    3, // 3
    4, // 4
    5, // 5
    6, // 6
    7, // 7
    // PORTB
    0, // 8
    1, // 9
    2, // 10
    3, // 11
    4, // 12
    5, // 13
    // PORTC
    0, // 14
    1, // 15
    2, // 16
    3, // 17
    4, // 18
    5, // 19
];

pub enum Port {
    B(u8),
    C(u8),
    D(u8),
}

pub fn map_pin_to_port(pin: u8) -> Port {
    match pin {
        n if n >= 8 && n <= 13 => Port::B(BIT_MAP[n as usize]),
        n if n >= 14 && n <= 19 => Port::C(BIT_MAP[n as usize]),
        n if n <= 7 => Port::D(BIT_MAP[n as usize]),
        _ => unreachable!(),
    }
}

pub fn config_pin_input(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => {
            crate::bit_clear!(DDRB, n);
            crate::bit_clear!(PORTB, n);
        }
        Port::C(n) => {
            crate::bit_clear!(DDRC, n);
            crate::bit_clear!(PORTC, n);
        }
        Port::D(n) => {
            crate::bit_clear!(DDRD, n);
            crate::bit_clear!(PORTD, n);
        }
    }
}

pub fn config_pin_input_pullup(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => {
            crate::bit_clear!(DDRB, n);
            crate::bit_set!(PORTB, n);
        }
        Port::C(n) => {
            crate::bit_clear!(DDRC, n);
            crate::bit_set!(PORTC, n);
        }
        Port::D(n) => {
            crate::bit_clear!(DDRD, n);
            crate::bit_set!(PORTD, n);
        }
    }
}

pub fn config_pin_output(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => crate::bit_set!(DDRB, n),
        Port::C(n) => crate::bit_set!(DDRC, n),
        Port::D(n) => crate::bit_set!(DDRD, n),
    }
}

pub fn pin_set(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => crate::bit_set!(PORTB, n),
        Port::C(n) => crate::bit_set!(PORTC, n),
        Port::D(n) => crate::bit_set!(PORTD, n),
    }
}

pub fn pin_clear(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => crate::bit_clear!(PORTB, n),
        Port::C(n) => crate::bit_clear!(PORTC, n),
        Port::D(n) => crate::bit_clear!(PORTD, n),
    }
}

pub fn pin_toggle(pin: u8) {
    match map_pin_to_port(pin) {
        Port::B(n) => crate::bit_toggle!(PORTB, n),
        Port::C(n) => crate::bit_toggle!(PORTC, n),
        Port::D(n) => crate::bit_toggle!(PORTD, n),
    }
}

pub fn pin_read(pin: u8) -> u8 {
    match map_pin_to_port(pin) {
        Port::B(n) => crate::bit_read!(PINB, n),
        Port::C(n) => crate::bit_read!(PINC, n),
        Port::D(n) => crate::bit_read!(PIND, n),
    }
}
