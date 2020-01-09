# klee-examples

This repo contains a set of usage examples for `klee-sys` low-level KLEE bindings.

## paths.rs

This example showcase the different path termintaiton conditions possible and their effect to KLEE test case generation.

## assume_assert.rs

This example showcase contract based verification, and the possibilies to extract proofs.

## struct.rs

This example show the case of partially symbolic structures.

## vcell_test.rs

Simple test to showcase low-level `vcell` access.

## register_test.rs

Simple test to showcase the use of the `volatile-register` abstraction.

## f401_minimal.rs

This example showcase the execution of code on the stm32f401 (and similar targets from the f4).

### dependencies

- stm32401 nucleo board or similiar with recent stlink firmware
- (recent) openocd (tested with version 0.10.0)
- arm-none-eabi-gdb (tested with version 0.8.3)
- llvm target thumbv7em-none-eabi (`rustup target add thumbv7em-none-eabi`)


## Licencse

Copyright Per Lindgren.

All rights reserved, use restricted for non-commercial purpose.