#![no_std]
#![no_main]

use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;
use vcell;

#[no_mangle]
fn main() {
    let mut vc = vcell::VolatileCell::new(0u32);
    match vc.get() {
        2 => klee_abort!(),
        1 => klee_abort!(),
        _ => (),
    };
}
