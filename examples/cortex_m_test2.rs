#![no_std]
#![no_main]

use klee_sys::klee_abort;
extern crate cortex_m;
extern crate panic_klee;

use cortex_m::peripheral::Peripherals;

static mut S: u32 = 100;

#[no_mangle]
fn main() {
    let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    let a = dwt.cyccnt.read();
    if a == unsafe { S } {
        panic!();
    }

    klee_abort!();
}

// #[no_mangle]
// pub fn some_interrupt() {
//     unsafe { S = 0 };
// }
