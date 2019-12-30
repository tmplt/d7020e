#![no_std]
#![no_main]

#[cfg(feature = "klee-analysis")]
use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};

#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;

#[cfg(not(feature = "klee-analysis"))]
#[no_mangle]
fn main() {
    let mut a = 0;
    panic!();
}

use vcell;
#[cfg(feature = "klee-analysis")]
#[no_mangle]
fn main() {
    let mut vc = vcell::VolatileCell::new(0u32);
    match vc.get() {
        2 => klee_abort!(),
        1 => klee_abort!(),
        _ => (),
    };
}
