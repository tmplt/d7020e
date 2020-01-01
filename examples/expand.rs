#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;

use cortex_m_rt::{entry, exception, pre_init};
use panic_klee as _;

use klee_sys::{klee_abort, klee_make_symbolic};

use cortex_m::peripheral::Peripherals;

static mut S: u32 = 100;

#[doc(hidden)]
#[export_name = "main_klee"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    __cortex_m_rt_main()
}
unsafe fn __cortex_m_rt_main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    let a = dwt.cyccnt.read();
    if a == unsafe { S } {
        ::core::panicking::panic("explicit panic", ::core::intrinsics::caller_location());
    }
    unsafe { ::klee_sys::ll::abort() };
}
#[export_name = "__pre_init"]
pub unsafe fn pre_init() {
    let mut a = 0;
    ::klee_sys::klee_make_symbolic(unsafe { &mut a }, unsafe {
        ::klee_sys::CStr::from_bytes_with_nul_unchecked("a\u{0}".as_bytes())
    });
    if a == 100 {
        unsafe { ::klee_sys::ll::abort() };
    }
}
#[doc(hidden)]
#[export_name = "DefaultHandler"]
fn __cortex_m_rt_DefaultHandler(_irqn: i16) -> ! {
    if _irqn > 255 {
        unsafe { ::klee_sys::ll::abort() };
    }
    unsafe { ::klee_sys::ll::abort() };
}
