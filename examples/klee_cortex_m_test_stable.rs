// > cargo klee --example klee_cortex_m_test -r -k -g -v --release
// ...
// KLEE: done: total instructions = 33
// KLEE: done: completed paths = 1
// KLEE: done: generated tests = 1
//
// The example, has just one path, leading up to an abort.
// The reason is that the progam unconditionally will panic, as
// taking the peripheral is only only allowed once (for soundness).
// (Internally this in tracked by a state variable.)
//
// This is very good news, that KLEE will detect the error at compile time.
//
// Let's remove the error to see if the example code is correct.
// (comment out the second `take`)
//
// > cargo klee --example klee_cortex_m_test -r -k -g -v
// ...
// KLEE: ERROR: examples/klee_cortex_m_test.rs:120: divide by zero
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 1454
// KLEE: done: completed paths = 6
// KLEE: done: generated tests = 4
// ..
//(gdb) shell ls klee-last
// assembly.ll  info  messages.txt  run.istats  run.stats  test000001.div.err  test000001.kquery  test000001.ktest  test000002.ktest  warnings.txt
//
// So we see that test000001.ktest was causing a division error,
// the other test case passed
//
// (gdb) set env KTEST_FILE=klee-last/test000001.ktest
// (gdb) run
// Starting program: /home/pln/rust/grepit/klee-examples/target/debug/examples/klee_cortex_m_test.replay
// Program received signal SIGFPE, Arithmetic exception.
// 0x0000555555555528 in main () at examples/klee_cortex_m_test.rs:133
// 133             let some_time_quota = unchecked_div(a, c - (b - 100));
//
// Let's look at the actual test
// (gdb) shell ktest-tool klee-last/test000001.ktest
// ktest file : 'klee-last/test000001.ktest'
// args       : ['/home/pln/rust/grepit/klee-examples/target/debug/examples/klee_cortex_m_test-d9038188a1e4d0b0.ll']
// num objects: 5
// object 0: name: 'PRIMASK'
// object 0: size: 4
// object 0: data: b'\x01\x01\x01\x01'
// object 0: hex : 0x01010101
// object 0: int : 16843009
// object 0: uint: 16843009
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
// (gdb) backtrace
// #0  0x0000555555555528 in main () at examples/klee_cortex_m_test.rs:133
// (gdb) print a
// $1 = 0
// (gdb) print b
// $2 = 100
// (gdb) print c
// $3 = 0
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
// Critical here is that KLEE spots that `(c - (b - 100)) = 0`, leading up to a division
// by zero error (satisfied by `c == 0` and `b == 100`)
//
// Notice here, that this error is spotted EVEN while we are telling
// Rust to use the primitive (intrinsic) division for "unchecked_div" performance.
//
#![no_std]
#![no_main]

// extern crate klee_sys;
extern crate cortex_m;
extern crate panic_klee;

use cortex_m::peripheral::Peripherals;

use core::ptr::read_volatile;

#[no_mangle]
fn main() {
    let peripherals = Peripherals::take().unwrap();
    // let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    dwt.enable_cycle_counter();
    let a = dwt.cyccnt.read();
    let b = dwt.cyccnt.read();
    let c = dwt.cyccnt.read();
    unsafe {
        let some_time_quota = a / (c - (b - 100));
        read_volatile(&some_time_quota); // prevent optimization
    }
}
