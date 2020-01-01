// > cargo klee --example klee_cortex_m_test_stable --release
// ...
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location

// KLEE: done: total instructions = 92
// KLEE: done: completed paths = 2
// KLEE: done: generated tests = 2
// ...
//
// In order to analyze hardware dependent code, hardware access are treated
// as a new symbolic value. In `cortex-m` we give symbolic names to core peripherals.
// The svd2rust generated PAC is currently given the symbolic name `vcell`. This
// might in the future change to giving the address to the register instead.
//
// A breakdown of the example:
// Behind the scenes the PRIMASK register is accessed, and given concrete value.
// (Under the hood, `Peripherals.take` executes in a global critical section.)
//
// This access is along the "happy path" towards the error, so any value would
// suffice (in this case 0x00000000 was selected by KLEE).
//
// The first `vcell` access: was done when enabling the cycle counter.
// The rest of accesses stem from reading `a`, `b`, and `c`.
// Critical here is that `a/(c - (b - 100)) can lead up to a number of errors.
// Notice that arithmics are "checked" by default in Rust.
// - (b - 100)            may overflow (u32 can never be negative)
// - (c - (b - 100))      may overflow (u32 can never be negative)
// - a / (c - (b - 100))  may render a division by zero
//
// Rust injects run-time verification code, that yields a panic!() on error.
// KLEE reports just one such error site (on the panic handler).
//
// > ktest-tool target/release/examples/klee-last/test000001.ktest
// ktest file : 'target/release/examples/klee-last/test000001.ktest'
// args       : ['/home/pln/rust/trustit/klee-examples/target/release/examples/klee_cortex_m_test_stable-be284cfed3691349.ll']
// num objects: 5
// object 0: name: 'PRIMASK'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x00'
// object 0: hex : 0x00000000
// object 0: int : 0
// object 0: uint: 0
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// object 2: name: 'vcell'
// object 2: size: 4
// object 2: data: b'\x00\x00\x00\x00'
// object 2: hex : 0x00000000
// object 2: int : 0
// object 2: uint: 0
// object 2: text: ....
// object 3: name: 'vcell'
// object 3: size: 4
// object 3: data: b'd\x00\x00\x00'
// object 3: hex : 0x64000000
// object 3: int : 100
// object 3: uint: 100
// object 3: text: d...
// object 4: name: 'vcell'
// object 4: size: 4
// object 4: data: b'\x00\x00\x00\x00'
// object 4: hex : 0x00000000
// object 4: int : 0
// object 4: uint: 0
// object 4: text: ....
//
// For this test, a = 0, b = 100, c = 0, which hits the division by zero.
//
// >  ktest-tool target/release/examples/klee-last/test000002.ktest
// ktest file : 'target/release/examples/klee-last/test000002.ktest'
// args       : ['/home/pln/rust/trustit/klee-examples/target/release/examples/klee_cortex_m_test_stable-be284cfed3691349.ll']
// num objects: 5
// object 0: name: 'PRIMASK'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x00'
// object 0: hex : 0x00000000
// object 0: int : 0
// object 0: uint: 0
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// object 2: name: 'vcell'
// object 2: size: 4
// object 2: data: b'\x00\x00\x00\x00'
// object 2: hex : 0x00000000
// object 2: int : 0
// object 2: uint: 0
// object 2: text: ....
// object 3: name: 'vcell'
// object 3: size: 4
// object 3: data: b'\x00\x00\x00\x00'
// object 3: hex : 0x00000000
// object 3: int : 0
// object 3: uint: 0
// object 3: text: ....
// object 4: name: 'vcell'
// object 4: size: 4
// object 4: data: b'\x00\x00\x00\x00'
// object 4: hex : 0x00000000
// object 4: int : 0
// object 4: uint: 0
// object 4: text: ....
//
// If we want to have a closer look on what's going on we can compile the project in
// without optimization (AKA dev mode or debug mode), and replay the tests in gdb.
// > cargo klee --example klee_cortex_m_test_stable -r -k -g
// ...
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/klee_cortex_m_test_stable.rs:100: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 1719
// KLEE: done: completed paths = 8
// KLEE: done: generated tests = 2
// ...
// For help, type "help".
// Type "apropos word" to search for commands related to "word"...
// Reading symbols from klee_cortex_m_test_stable.replay...
//
// (gdb) set env KTEST_FILE=klee-last/test000001.ktest
// (gdb) run
// Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/klee_cortex_m_test_stable.replay

// Program received signal SIGABRT, Aborted.
// 0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// (gdb) backtrace
// #0  0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// #1  0x00007ffff7dbd897 in abort () from /usr/lib/libc.so.6
// #2  0x000055555555512b in rust_begin_unwind (_info=0x7fffffffd1b8) at /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8
// #3  0x00005555555557ed in core::panicking::panic_fmt () at src/libcore/panicking.rs:139
// #4  0x0000555555555859 in core::panicking::panic () at src/libcore/panicking.rs:70
// #5  0x0000555555555389 in main () at examples/klee_cortex_m_test_stable.rs:96
//
// (gdb) frame 5
// #5  0x0000555555555389 in main () at examples/klee_cortex_m_test_stable.rs:96
// 96          let some_time_quota = a / (c - (b - 100));
// (gdb) p a
// $1 = 0
// (gdb) p b
// $2 = 0
// (gdb) p c
// $3 = 0
//
// In this case (b - 100) overflows. (In debug mode we have checked not wrapping arithmetics.)
//
// (gdb) shell ktest-tool klee-last/test000002.ktest
// Program received signal SIGABRT, Aborted.
// 0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// (gdb) backtrace
// #0  0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// #1  0x00007ffff7dbd897 in abort () from /usr/lib/libc.so.6
// #2  0x00005555555553dd in main () at examples/klee_cortex_m_test_stable.rs:100
// (gdb) frame 2
// #2  0x00005555555553dd in main () at examples/klee_cortex_m_test_stable.rs:100
// 100         klee_abort!();
// (gdb) p a
// $4 = 0
// (gdb) p b
// $5 = 276938064
// (gdb) p c
// $6 = 1350613988
//
// In this case there was no arithmetic error and we hit the `klee_abort!()` at the last line.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_klee;

use cortex_m::peripheral::Peripherals;
use klee_sys::klee_abort;

use core::ptr::read_volatile;
#[no_mangle]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    // let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    dwt.enable_cycle_counter();
    let a: u32 = dwt.cyccnt.read();
    let b: u32 = dwt.cyccnt.read();
    let c: u32 = dwt.cyccnt.read();
    let some_time_quota = a / (c - (b - 100));
    unsafe {
        read_volatile(&some_time_quota); // prevent optimization
    }
    klee_abort!();
}
