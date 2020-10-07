#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {

    // Macro for printing to Host stdout with newline (\n) at the end.
    hprintln!("Hello, world!").unwrap();

    loop {
        // your code goes here
    }
}
