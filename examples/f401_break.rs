// minimal example for the stm32-f401 (and the f4 series)
//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f4::stm32f401 as stm32;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::bkpt();

    loop {}
}

// See RM0368 Reference Manual for details
// https://www.st.com/content/ccc/resource/technical/document/reference_manual/5d/b1/ef/b2/a1/66/40/80/DM00096844.pdf/files/DM00096844.pdf/jcr:content/translations/en.DM00096844.pdf
//
// Memory map is given in `memory.x`, in particular we define:
// 96k RAM starting at 0x2000_0000, and
// 256k Flash starting at 0x0800_0000
//
// Run in separate terminal:
// openocd -f openocd.cfg
//
// cargo run --example f401_minimal --features f4 --target thumbv7em-none-eabihf
// This is the way!
