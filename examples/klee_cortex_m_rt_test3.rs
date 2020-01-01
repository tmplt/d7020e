#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, pre_init};
use panic_klee as _;

use klee_sys::{klee_abort, klee_make_symbolic};

use cortex_m::peripheral::Peripherals;

static mut S: u32 = 100;

#[entry]
unsafe fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    let a = dwt.cyccnt.read();
    if a == unsafe { S } {
        panic!();
    }

    klee_abort!();
}

#[pre_init]
unsafe fn pre_init() {
    let mut a = 0;
    klee_make_symbolic!(&mut a, "a");
    if a == 100 {
        klee_abort!();
    }
}

#[exception]
fn DefaultHandler(irqn: i16) -> ! {
    static mut X: i16 = 0;
    if irqn > 255 {
        unsafe { core::ptr::write_volatile(&mut X, irqn) };
        klee_abort!();
    } else {
        unsafe { core::ptr::write_volatile(&mut X, 0) };
        klee_abort!();
    }
}
