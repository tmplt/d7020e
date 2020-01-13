// minimal example for the stm32-f401 (and the f4 series)
//! Prints "Hello, world!" on the host console using semihosting
#![feature(asm)]
#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f4::stm32f401 as stm32;

use cortex_m::{asm, bkpt, iprintln};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut x: u32 = 54;
    // klee_make_symbolic(&mut x);
    // while x == 0 {}
    // // asm::bkpt();
    //
    //
    bkpt!(1);
    asm::nop();
    asm::nop();

    bkpt!(2);
    asm::nop();

    bkpt!();
    loop {
        asm::nop();
    }
}

#[inline(always)]
fn klee_make_symbolic<T>(data: &mut T) {
    asm::bkpt();
    // unsafe { klee_bkpt(data as *mut T as *mut core::ffi::c_void) };
}

#[no_mangle]
pub extern "C" fn klee_bkpt(data: *mut core::ffi::c_void) {
    //*data = 0;
    asm::bkpt();
}

// extern "C" {
//     pub fn klee_bkpt(ptr: *mut core::ffi::c_void, // pointer to the data
//     );
// }
