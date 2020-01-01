//! examples/init.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;
// use rtfm;
// use cortex_m_rt as _;
// use lm3s6965 as _;
// extern crate cortex_m_rt;
// extern crate lm3s6965;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        //static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        // let _x: &'static mut u32 = X;
        //    fn_to_test();
    }
};
