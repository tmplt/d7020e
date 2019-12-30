// showcase volatile register
// This is the underlying abstraction to all register accesses using the
// embedded Rust ecosystem.
//
// When analyzed by KLEE, we make the return value symbolic, thus each access
// givs a new unique symbol. Even if we write a value to it, the next read
// will still be treated as a new symbol. That might be overly pessimistic
// but is a safe approximation for the worst case behavior of the hardware.
//
// > cargo klee --example register_test --release
// ...
// KLEE: ERROR: /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-abort-0.3.2/src/lib.rs:49: abort failure
// KLEE: NOTE: now ignoring this error at this location

// KLEE: done: total instructions = 33
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
//
// > ls target/release/examples/klee-last/
// assembly.ll  info  messages.txt  run.istats  run.stats  test000001.ktest  test000002.ktest  test000003.abort.err  test000003.kquery  test000003.ktest  warnings.txt
//
// We see that KLEE spotted the test3 hits unreachable (and thus panics)
//
// Let's look at the test cases separately:
//
// test1 passed:
// ktest-tool target/release/examples/klee-last/test000001.ktest
// ktest file : 'target/release/examples/klee-last/test000001.ktest'
// args       : ['...']
// num objects: 1
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x00'
// object 0: hex : 0x00000000
// object 0: int : 0
// object 0: uint: 0
// object 0: text: ....
// If the first read of the register is not 1 then we are ok.
//
// The second test also passed.
// ktest-tool target/release/examples/klee-last/test000002.ktest
// ktest file : 'target/release/examples/klee-last/test000002.ktest'
// args       : ['...']
// num objects: 2
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x01\x00\x00\x00'
// object 0: hex : 0x01000000
// object 0: int : 1
// object 0: uint: 1
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// Here we are saved by the second reading of the register NOT giving
// returning 2.
//
// The third test gives the error.
// ktest-tool target/release/examples/klee-last/test000003.ktest
// ktest file : 'target/release/examples/klee-last/test000003.ktest'
// args       : ['...']
// num objects: 2
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x01\x00\x00\x00'
// object 0: hex : 0x01000000
// object 0: int : 1
// object 0: uint: 1
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x02\x00\x00\x00'
// object 1: hex : 0x02000000
// object 1: int : 2
// object 1: uint: 2
// object 1: text: ....
//
// The first read gives 1, the second 2, and we hit unreacable.

#![no_std]
#![no_main]

use panic_klee as _;
use volatile_register::RW;

#[no_mangle]
fn main() {
    // we emulate a read/write hardware register (rw)
    let rw: RW<u32> = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    // reading will render a symbolic value of type u32
    if rw.read() == 1 {
        // we emulate a write to the hardware register
        unsafe { rw.write(0) };
        // this read is still treated as a new symbolic value of type u32
        if rw.read() == 2 {
            // will generate a panic!() if reached.
            unreachable!();
        }
    }
}
