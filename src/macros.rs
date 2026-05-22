#[macro_export]
macro_rules! bit_set {
    ($r:expr, $n:expr) => {
        unsafe {
            let val = core::ptr::read_volatile($r);
            core::ptr::write_volatile($r, val | (1 << $n));
        }
    };
}

#[macro_export]
macro_rules! bit_clear {
    ($r:expr, $n:expr) => {
        unsafe {
            let val = core::ptr::read_volatile($r);
            core::ptr::write_volatile($r, val & !(1 << $n));
        }
    };
}

#[macro_export]
macro_rules! bit_toggle {
    ($r:expr, $n:expr) => {
        unsafe {
            let val = core::ptr::read_volatile($r);
            core::ptr::write_volatile($r, val ^ (1 << $n));
        }
    };
}

#[macro_export]
macro_rules! bit_read {
    ($r:expr, $n:expr) => {
        unsafe { (core::ptr::read_volatile($r) >> $n) & 1 }
    };
}
