#![no_std]
#![no_main]

#[cfg(feature = "klee-analysis")]
use klee_sys::{klee_abort, klee_make_symbolic};

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;

#[cfg(not(feature = "klee-analysis"))]
#[no_mangle]
fn main() {
    let mut a = 0;
    panic!();
}

#[cfg(feature = "klee-analysis")]
#[no_mangle]
fn main() {
    let mut a = 0;
    klee_make_symbolic(&mut a, "a");
    match a {
        0 => klee_abort(),
        2 => klee_abort(),
        _ => (),
    };
    panic();
}
