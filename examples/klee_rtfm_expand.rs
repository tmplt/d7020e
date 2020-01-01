// klee-analysis
#![feature(prelude_import)]
#![feature(rustc_private)]
//! examples/init.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;

// use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;
// use rtfm;

#[no_mangle]
fn main() -> ! {
    loop {}
}
//static mut X: u32 = 0;

// Safe access to local `static mut` variable
// let _x: &'static mut u32 = X;
//    fn_to_test();
