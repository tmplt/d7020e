#![no_std]
#![no_main]

#[macro_use]
extern crate klee_sys;

#[cfg(not(feature = "klee-analysis"))]
extern crate panic_halt;

struct A {
    a: u8,
    b: u32,
}

#[no_mangle]
fn main() {
    let mut a = 0;
    // ksymbol!(&mut a, "a");
    let mut u = A { a: a, b: 0 };
    u.b = 7;
    let _ = f2(f1(u.a));
}

// add 1 wrapping instead of panic
fn f1(u: u8) -> u8 {
    u.wrapping_add(1)
}

fn f2(u: u8) -> u8 {
    100 / u
}
