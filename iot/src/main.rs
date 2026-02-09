#![allow(clippy::missing_safety_doc)]
#![allow(unsafe_op_in_unsafe_fn)]
#![no_std]
#![no_main]

use panic_abort as _;

mod support;
mod types;

use support::exit;

#[unsafe(no_mangle)]
pub unsafe fn _start() -> ! {
    exit(0)
}
