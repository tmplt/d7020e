// example to show conditional compilation of hybrid applications
// compiling both for klee analysis and just as bare metal
//
#![no_std]
#![no_main]

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

#[cfg(feature = "klee-analysis")]
use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
#[cfg(feature = "klee-analysis")]
#[no_mangle]
fn main() {
    let mut a = 0;
    klee_make_symbolic!(&mut a, "a");
    // Rust panic on a == 200;
    let _ = 100 / (a - 200);
    match a {
        0 => klee_abort!(),
        1 => klee_abort!(),
        2 => klee_abort!(),
        3 => panic!("3"), // just one instance of panic! will be spotted
        4 => klee_assert!(false),
        5 => klee_assert_eq!(false, true),
        6 => klee_assert_eq!(false, true),
        _ => (),
    };
    panic!("at end"); // just one instane of panic! will be spotted
}
