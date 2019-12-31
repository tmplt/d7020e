#![no_std]
#![no_main]

// extern crate cortex_m;
use cortex_m_rt::{entry, pre_init};
use panic_klee as _;

use klee_sys::klee_abort;

use cortex_m::peripheral::Peripherals;

// #[entry]
// unsafe fn main() -> ! {
//     let peripherals = Peripherals::take().unwrap();
//     // let peripherals = Peripherals::take().unwrap();
//     let mut dwt = peripherals.DWT;
//     dwt.enable_cycle_counter();
//     let a = dwt.cyccnt.read();
//     if a == 100 {
//         klee_abort!();
//     }
//     klee_abort!();
// }

#[pre_init]
unsafe fn pre_init() {}
