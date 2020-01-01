#![feature(core_intrinsics)]
#![feature(rustc_private)]
#![feature(core_panic)]
#![feature(prelude_import)]
//! examples/init.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[cfg(feature = "klee-analysis")]
use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};

#[no_mangle]
fn main() -> ! {
    fn_to_test();
    loop {}
}

// Safe access to local `static mut` variable

#[cfg(feature = "klee-analysis")]
fn fn_to_test() {
    let mut a = 0; // just one instance of panic! will be spotted
                   // just one instane of panic! will be spotted

    ::klee_sys::klee_make_symbolic(unsafe { &mut a }, unsafe {
        ::klee_sys::CStr::from_bytes_with_nul_unchecked("a\u{0}".as_bytes())
    });
    match a {
        0 => unsafe { ::klee_sys::ll::abort() },
        1 => unsafe { ::klee_sys::ll::abort() },
        2 => ::core::panicking::panic("explicit panic", ::core::intrinsics::caller_location()),
        3 => ::core::panicking::panic("3", ::core::intrinsics::caller_location()),
        4 => {
            if !false {
                unsafe { ::klee_sys::ll::abort() };
            }
        }
        5 => {
            if !(false == true) {
                unsafe { ::klee_sys::ll::abort() };
            }
        }
        6 => {
            if !(false == true) {
                unsafe { ::klee_sys::ll::abort() };
            }
        }
        _ => (),
    };
    ::core::panicking::panic("at end", ::core::intrinsics::caller_location());
}
