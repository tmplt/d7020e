// example that shows different types of path termination
// and their effect to KLEE test case generation
#![no_std]
#![no_main]

use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;

#[no_mangle]
fn main() {
    let mut a = 0;
    klee_make_symbolic!(&mut a, "a");
    // Rust panic on a == 200;
    let _ = 100 / (a - 200);

    let _ = match a {
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
